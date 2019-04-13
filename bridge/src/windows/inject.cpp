#include <Windows.h>
#include <psapi.h>
#include <cstdint>
// #include <cstdio>
#define printf()

__forceinline HMODULE GetProcessModuleAddress(HANDLE process, const wchar_t *dllname)
{
  HMODULE arr[100];
  wchar_t filename[MAX_PATH];
  DWORD required;
  if (TRUE == EnumProcessModules(process, arr, sizeof(arr), &required))
  {
    size_t n = required / sizeof(HMODULE);
    for (size_t i = 0; i < n; ++i)
    {
      if (size_t filenameLen = GetModuleFileNameExW(process, arr[i], filename, sizeof(filename) / sizeof(wchar_t)))
      {
        if (0 == _wcsicmp(&(filename[filenameLen - wcslen(dllname)]), dllname))
          return arr[i];
      }
    }
  }
  return NULL;
}

__forceinline BOOL RemoteLoadLibrary(HANDLE hProcess, LPWSTR lpLibPath)
{
  HANDLE hThread;
  HMODULE hKernel32; /* = GetModuleHandle ( "Kernel32" ); */
  LPVOID lpvLibPath; /* Pointer to the remote address */

  size_t path_size = (wcslen(lpLibPath) + 1) * sizeof(wchar_t);

  /* Allocate the space required for the string in our process */
  if ((lpvLibPath = VirtualAllocEx(hProcess, NULL, path_size, MEM_COMMIT, PAGE_READWRITE)))
  {
    /* Remotely copy lpLibPath to the pointed address of lpvLibPath */
    WriteProcessMemory(hProcess, lpvLibPath, (LPVOID)lpLibPath, path_size, NULL);

    /* Get Kernel32 from the memory.  Providing you didn't fuck your system, this should almost always work */
    GetModuleHandleExA(0, "Kernel32", &hKernel32);

    auto remote_base = GetProcessModuleAddress(hProcess, L"Kernel32.dll");

    printf("remote base = %x\n", remote_base);

    if (remote_base == NULL)
    {
      return FALSE;
    }

    auto f_addr = GetProcAddress(hKernel32, "LoadLibraryW");

    printf("LoadLibraryW addr = %x\n", f_addr);

    auto f_offset = (uint32_t)f_addr - (uint32_t)hKernel32;

    printf("LoadLibraryW offset = %x\n", f_offset);

    auto f_remote_addr = (uint32_t)remote_base + f_offset;

    printf("f_remote_addr = %x\n", f_remote_addr);

    /* Execute LoadLibrary in our process remotely and see if the thread was executed successfuly */
    if ((hThread = CreateRemoteThread(hProcess, NULL, 0, (LPTHREAD_START_ROUTINE)(f_remote_addr), lpvLibPath, 0, NULL)))
    {
      /* Free the remotely allocated string */
      VirtualFreeEx(hProcess, lpvLibPath, path_size, MEM_RELEASE);

      /* Wait for the thread to finish and return.  When that's done, we'll have the DLL loaded in the process space */
      WaitForSingleObject(hThread, INFINITE);

      return TRUE;
    }
    return FALSE;
  }
  return FALSE;
}

enum class InjectResult
{
  Ok = 0,
  GetPid = 1,
  OpenProcess = 2,
  DuplicateHandle = 3,
  RemoteLoadLibrary = 4
};

extern "C" InjectResult inject_pid_and_wait(DWORD pid, LPWSTR lpLibPath)
{
  HANDLE process = OpenProcess(1, FALSE, pid);
  HANDLE currentProcess = GetCurrentProcess();
  if (process)
  {
    printf("Process opened (1): 0x%X\n", (unsigned int)process);
    HANDLE newProcess;
    if (DuplicateHandle(currentProcess, process, currentProcess, &newProcess, PROCESS_ALL_ACCESS, FALSE, DUPLICATE_CLOSE_SOURCE))
    {
      printf("Process opened (2): 0x%X\n", (unsigned int)newProcess);
      printf("Injecting...");
      if (RemoteLoadLibrary(newProcess, lpLibPath))
      {
        printf("Injected.\n");
        WaitForSingleObject(newProcess, INFINITE);
        CloseHandle(newProcess);
        printf("Process ended.\n");
        return InjectResult::Ok;
      }
      else
      {
        CloseHandle(newProcess);
        return InjectResult::RemoteLoadLibrary;
      }
    }
    else
    {
      return InjectResult::DuplicateHandle;
    }
  }
  else
  {
    return InjectResult::OpenProcess;
  }
}

extern "C" InjectResult inject_and_wait(HWND win, LPWSTR lpLibPath)
{
  printf("Find window: %X\n", (unsigned int)win);
  DWORD pid;
  GetWindowThreadProcessId(win, &pid);
  if (pid)
  {
    return inject_pid_and_wait(pid, lpLibPath);
  }
  else
  {
    return InjectResult::GetPid;
  }
}