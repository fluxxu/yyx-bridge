use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

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

#[cfg(feature = "fg")]
type FindPidByExeNameCallback = extern "C" fn(*const c_char) -> bool;

extern "C" {
  fn get_version(dst: *mut c_char) -> bool;
  fn get_code_section_impl(section: *mut Section) -> bool;
  #[cfg(feature = "fg")]
  fn find_pid_by_path(exe_name: *const c_char, callback: FindPidByExeNameCallback) -> u32;
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

// Facebook Gameroom
#[cfg(feature = "fg")]
pub fn find_fg_pid() -> u32 {
  use bridge_derive::secret_string;
  use std::ffi::CString;
  let name_cstring = CString::new(secret_string!("client.exe")).unwrap();
  unsafe { find_pid_by_path(name_cstring.as_ptr(), find_fg_pid_callback) }
}

#[cfg(feature = "fg")]
extern "C" fn find_fg_pid_callback(name: *const c_char) -> bool {
  use bridge_derive::secret_string;
  unsafe {
    use std::ffi::CStr;
    let name_str = CStr::from_ptr(name).to_string_lossy();
    // println!("name = {}", name_str);
    name_str.ends_with("client.exe") && name_str.contains(&secret_string!("638864706286069"))
  }
}
