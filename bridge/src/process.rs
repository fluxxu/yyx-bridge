use std::ffi::CStr;
use std::os::raw::c_char;
use std::{ptr};

#[repr(C)]
#[derive(Debug)]
pub struct Section {
  pub base: *const u8,
  pub size: usize,
}

// impl Section {
//   pub unsafe fn as_slice(&self) -> &[u8] {
//     slice::from_raw_parts(self.base, self.size)
//   }
// }

extern "C" {
  fn get_version(dst: *mut c_char) -> bool;
  fn get_code_section_impl(section: *mut Section) -> bool;
}

pub fn get_code_section() -> Option<Section> {
  assert_eq!(::std::mem::size_of::<Section>(), 8);

  let mut section = Section {
    base: ptr::null(),
    size: 0,
  };
  let ok = unsafe { get_code_section_impl(&mut section) };
  if ok {
    Some(section)
  } else {
    None
  }
}

#[inline(always)]
pub fn get_version_string() -> String {
  let mut buf: [i8; 64] = [0; 64];
  unsafe {
    if get_version(buf.as_mut_ptr()) {
      CStr::from_ptr(buf.as_ptr())
        .to_str()
        .ok()
        .unwrap_or("")
        .to_owned()
    } else {
      "".to_owned()
    }
  }
}
