use std::ffi::c_void;
use std::os::raw::c_char;

pub const YYS_WINDOW_CLASS: &str = "Win32Window";
pub const YYS_WINDOW_TITLE: &[u8] = &[
  210, 245, 209, 244, 202, 166, 45, 205, 248, 210, 215, 211, 206, 207, 183,
];

extern "C" {
  pub fn find_window_by_class_and_title(
    class: *const c_char,
    title: *const c_char,
  ) -> *const c_void;
}

#[test]
fn test_find_window_by_class_and_title() {
  use std::ffi::CString;
  let class = CString::new("Win32Window0").unwrap();
  let title = CString::new(YYS_WINDOW_TITLE).unwrap();
  let hwnd = unsafe { find_window_by_class_and_title(class.as_ptr(), title.as_ptr()) };
  println!("hwnd = {:?}", hwnd)
}
