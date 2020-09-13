use serde::Serialize;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
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

  pub fn err_with_data(message: &str, raw: Vec<u8>) -> Self {
    PullResult {
      is_ok: false,
      error_message: CString::new(message).unwrap().into_raw(),
      data_json: CString::new(raw).unwrap().into_raw(),
    }
  }

  pub fn get_error_message(&self) -> Option<String> {
    use std::ffi::CString;
    if self.error_message != ptr::null_mut() {
      let cstr = unsafe { CString::from_raw(self.error_message) };
      let v = cstr.to_string_lossy().to_string();
      cstr.into_raw();
      Some(v)
    } else {
      None
    }
  }

  pub fn get_data_json(&self) -> Option<String> {
    use std::ffi::CString;
    if self.data_json != ptr::null_mut() {
      let cstr = unsafe { CString::from_raw(self.data_json) };
      let v = cstr.to_string_lossy().to_string();
      cstr.into_raw();
      Some(v)
    } else {
      None
    }
  }
}

impl Drop for PullResult {
  fn drop(&mut self) {
    unsafe {
      if self.error_message != ptr::null_mut() {
        CString::from_raw(self.error_message);
      }
      if self.data_json != ptr::null_mut() {
        CString::from_raw(self.data_json);
      }
    }
  }
}
