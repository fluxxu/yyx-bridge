use serde::Serialize;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

pub struct PullResult {
  pub is_ok: bool,
  pub error_message: *mut c_char,
  pub data_json: *mut c_char,
}

impl PullResult {
  pub fn ok<T: Serialize>(data: T) -> Self {
    use serde_json;
    let json = serde_json::to_string(&data).unwrap();
    PullResult {
      is_ok: true,
      error_message: ptr::null_mut(),
      data_json: CString::new(json).unwrap().into_raw(),
    }
  }

  pub fn err(message: &str) -> Self {
    PullResult {
      is_ok: false,
      error_message: CString::new(message).unwrap().into_raw(),
      data_json: ptr::null_mut(),
    }
  }
}

pub fn run_client() {}

pub fn run_server() -> PullResult {
  PullResult::err("not implemented.")
}

pub unsafe fn free_result(result: PullResult) {
  if result.error_message != ptr::null_mut() {
    CString::from_raw(result.error_message);
  }
  if result.data_json != ptr::null_mut() {
    CString::from_raw(result.data_json);
  }
}
