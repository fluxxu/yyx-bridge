use std::ffi::c_void;
use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
  fn create_pattern(pattern_str: *const c_char) -> *const c_void;
  fn destroy_pattern(pattern: *const c_void);
  fn find_pattern(pattern: *const c_void, range_begin: *const u8, size: usize) -> *const u8;
  #[allow(unused)]
  fn match_pattern(pattern: *const c_void, range_begin: *const u8, size: usize) -> bool;
}

pub struct PatternFinder(*const c_void);

impl PatternFinder {
  pub fn new(p: &str) -> Self {
    let cstr = CString::new(p).unwrap();
    unsafe { PatternFinder(create_pattern(cstr.as_ptr())) }
  }

  pub fn find_pattern(&self, range_begin: *const u8, size: usize) -> Option<*const u8> {
    let p = unsafe { find_pattern(self.0, range_begin, size) };
    if p.is_null() {
      None
    } else {
      Some(p)
    }
  }

  #[allow(unused)]
  pub fn match_pattern(&self, range_begin: *const u8, size: usize) -> bool {
    unsafe { match_pattern(self.0, range_begin, size) }
  }
}

impl Drop for PatternFinder {
  fn drop(&mut self) {
    unsafe { destroy_pattern(self.0) }
  }
}

#[test]
fn test_create_pattern() {
  use std::ffi::CString;
  let pattern_str = CString::new("33 99 FF ?? 00").unwrap();
  let data: [u8; 5] = [0x33, 0x99, 0xFF, 0x88, 0x00];
  let result = unsafe {
    let pattern = create_pattern(pattern_str.as_ptr());
    match_pattern(pattern, &data as *const u8, data.len())
  };
  assert!(result, "should match");

  let data: [u8; 5] = [0x33, 0x99, 0xFF, 0x88, 0x11];
  let result = unsafe {
    let pattern = create_pattern(pattern_str.as_ptr());
    match_pattern(pattern, &data as *const u8, data.len())
  };
  assert!(!result, "should not match");
}
