#include <Windows.h>
#include <cstdio>

struct Section
{
  void *base;
  DWORD size;
};

extern "C" bool get_code_section_impl(Section *out)
{
  HMODULE hModule = GetModuleHandle(NULL);

  PIMAGE_DOS_HEADER DosHeader = (PIMAGE_DOS_HEADER)hModule;
  PIMAGE_NT_HEADERS PeHeader = (PIMAGE_NT_HEADERS)((DWORD)DosHeader + DosHeader->e_lfanew);

  DWORD CodeAddress = (DWORD)DosHeader + PeHeader->OptionalHeader.BaseOfCode;
  DWORD CodeLength = PeHeader->OptionalHeader.SizeOfCode;

  if (!CodeAddress || !CodeLength)
    return false;
  out->base = (void *)CodeAddress;
  out->size = CodeLength;
  return true;
}

static const int kMaxVersionStringSize = 64;
extern "C" bool get_version(char dst[kMaxVersionStringSize])
{
  char fileName[MAX_PATH];
  DWORD rv = GetModuleFileNameA(NULL, fileName, MAX_PATH);
  if (rv == 0)
  {
    return false;
  }

  rv = GetFileVersionInfoSizeA(fileName, NULL);
  if (rv)
  {
    char *buffer = (char *)malloc(rv + 1);
    if (GetFileVersionInfoA(fileName, NULL, rv, buffer))
    {
      VS_FIXEDFILEINFO *pFixedInfo;
      unsigned int infoLength;
      if (VerQueryValueA(buffer, "\\", reinterpret_cast<LPVOID *>(&pFixedInfo), &infoLength))
      {
        sprintf_s(dst, kMaxVersionStringSize, "%u.%u.%u.%u",
                  pFixedInfo->dwFileVersionMS >> 0x10,
                  pFixedInfo->dwFileVersionMS & 0xFFFF,
                  pFixedInfo->dwFileVersionLS >> 0x10,
                  pFixedInfo->dwFileVersionLS & 0xFFFF);
      }
    }
    free(buffer);
  }
  return rv != 0;
}