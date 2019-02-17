use crate::result::*;
use std::ffi::c_void;
use std::os::raw::c_char;
use std::path::Path;

#[derive(Debug)]
pub enum InjectError {
  YYSWindowNotFound,
}

extern "C" {
  fn inject(win: *const c_void, dll_path: *const c_char) -> *const c_void;
}

pub fn inject_dll_to_yys<P: AsRef<Path>>(path: P) -> UtilsResult<()> {
  use super::window::*;
  use std::ffi::CString;
  let class = CString::new(YYS_WINDOW_CLASS).unwrap();
  let title = CString::new(YYS_WINDOW_TITLE).unwrap();
  let dll_path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
  let hwnd = unsafe { find_window_by_class_and_title(class.as_ptr(), title.as_ptr()) };
  if hwnd.is_null() {
    return Err(UtilsError::Inject(InjectError::YYSWindowNotFound));
  }

  unsafe { inject(hwnd, dll_path.as_ptr()) };
  Ok(())
}
