#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate failure_derive;

#[cfg(debug_assertions)]
#[macro_use]
extern crate log;

#[cfg(not(debug_assertions))]
macro_rules! debug {
  ($($expr:expr),*) => {
    ()
  };
}

use std::ffi::CString;
use std::os::raw::c_char;

mod result;
mod version;
pub use self::result::PullResult;

mod emulator;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
#[no_mangle]
pub extern "stdcall" fn DllMain(
  hinst_dll: HINSTANCE,
  fdw_reason: u32,
  _: *mut winapi::ctypes::c_void,
) {
  match fdw_reason {
    1 => windows::run(hinst_dll),
    0 => {
      debug!("bridge module unloaded.");
    }
    _ => {}
  }
}

#[cfg(target_os = "windows")]
#[no_mangle]
pub unsafe extern "C" fn pull_run() -> PullResult {
  windows::run_server()
}

#[no_mangle]
pub unsafe extern "C" fn pull_free(result: PullResult) {
  drop(result)
}

#[no_mangle]
pub unsafe extern "C" fn version_get() -> *mut c_char {
  CString::new(version::VERSION).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn version_free(v: *mut c_char) {
  CString::from_raw(v);
}
