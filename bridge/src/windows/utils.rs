use std::path::PathBuf;
use winapi::shared::minwindef::HINSTANCE;

pub fn get_module_path(handle: HINSTANCE) -> PathBuf {
  use std::ffi::OsString;
  use std::os::windows::prelude::*;
  use winapi::um::libloaderapi::GetModuleFileNameW;
  let mut buf = [0; 255];
  let n = unsafe { GetModuleFileNameW(handle, buf.as_mut_ptr(), 255) };
  assert_ne!(n, 255);
  PathBuf::from(OsString::from_wide(&buf[0..(n as usize)]))
}
