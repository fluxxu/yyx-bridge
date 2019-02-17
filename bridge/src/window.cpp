#include <cstdio>
#include <Windows.h>

struct FindWindowContext
{
  DWORD process_id;
  PCHAR class_name;
  PCHAR title;
  HWND out;
};

BOOL CALLBACK find_window_callback(HWND hwnd, FindWindowContext *ctx)
{
  if (ctx->process_id)
  {
    DWORD pid = 0;
    GetWindowThreadProcessId(hwnd, &pid);
    if (pid != ctx->process_id)
      return TRUE;
  }

  if (ctx->class_name)
  {
    CHAR ClassName[1024] = {0};
    GetClassNameA(hwnd, ClassName, sizeof(ClassName));

    if (_stricmp(ctx->class_name, ClassName) != 0)
      return TRUE;
  }

  if (ctx->title)
  {
    CHAR title[1024] = {0};
    GetWindowTextA(hwnd, title, sizeof(title));

    // for (int i = 0; title[i]; i++)
    // {
    //   printf("%u, ", (char)title[i] & 0xff);
    // }

    if (_stricmp(ctx->title, title) != 0)
      return TRUE;
  }

  ctx->out = hwnd;
  return FALSE;
}

extern "C" HWND find_window_by_class_and_title(PCHAR class_name, PCHAR title)
{
  FindWindowContext ctx = {0};

  ctx.class_name = class_name;
  ctx.title = title;

  EnumWindows((WNDENUMPROC)find_window_callback, (LPARAM)&ctx);

  return ctx.out;
}