use crate::result::*;
use std::ffi::c_void;
use std::os::raw::c_char;
use std::path::Path;

#[derive(Debug)]
pub enum InjectError {
  YYSWindowNotFound,
  InjectError(i32),
}

extern "C" {
  fn inject_and_wait(win: *const c_void, dll_path: *const c_char) -> i32;
}

pub fn inject_dll_to_yys<P: AsRef<Path>>(path: P) -> BridgeResult<()> {
  use super::window::*;
  use std::ffi::CString;
  let class = CString::new(YYS_WINDOW_CLASS).unwrap();
  let title = CString::new(YYS_WINDOW_TITLE).unwrap();
  let dll_path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
  let hwnd = unsafe { find_window_by_class_and_title(class.as_ptr(), title.as_ptr()) };
  if hwnd.is_null() {
    return Err(BridgeError::Inject(InjectError::YYSWindowNotFound));
  }

  debug!("window found: {:?}", hwnd);

  let res = unsafe { inject_and_wait(hwnd, dll_path.as_ptr()) };
  if res == 0 {
    Ok(())
  } else {
    Err(BridgeError::Inject(InjectError::InjectError(res)))
  }
}
