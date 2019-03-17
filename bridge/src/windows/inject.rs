use super::result::*;
use std::ffi::c_void;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;

#[derive(Debug)]
pub enum InjectError {
  YYSWindowNotFound,
  InjectError(i32),
}

extern "C" {
  fn inject_and_wait(win: *const c_void, dll_path: *const u16) -> i32;
}

pub fn inject_dll_to_yys<P: AsRef<Path>>(path: P) -> BridgeResult<()> {
  use super::window::*;
  use std::ffi::CString;
  let class = CString::new(YYS_WINDOW_CLASS).unwrap();
  let title = CString::new(YYS_WINDOW_TITLE).unwrap();
  let hwnd = unsafe { find_window_by_class_and_title(class.as_ptr(), title.as_ptr()) };
  if hwnd.is_null() {
    return Err(BridgeError::Inject(InjectError::YYSWindowNotFound));
  }

  debug!("window found: {:?}", hwnd);

  let path_wstr: Vec<u16> = path
    .as_ref()
    .as_os_str()
    .encode_wide()
    .chain(::std::iter::once(0))
    .collect();

  let res = unsafe { inject_and_wait(hwnd, path_wstr.as_ptr()) };
  if res == 0 {
    Ok(())
  } else {
    Err(BridgeError::Inject(InjectError::InjectError(res)))
  }
}
