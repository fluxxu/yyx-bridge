use std::os::raw::c_char;
use std::ptr;

#[derive(Debug)]
pub struct PullResult {
  pub is_ok: bool,
  pub error_message: *mut c_char,
  pub data_json: *mut c_char,
}

impl PullResult {
  fn get_error_message(&self) -> Option<String> {
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

  fn get_data_json(&self) -> Option<String> {
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

fn main() {
  let lib = libloading::Library::new("bridge.dll").unwrap();
  unsafe {
    let pull_run: libloading::Symbol<unsafe extern "C" fn() -> PullResult> =
      lib.get(b"pull_run").unwrap();
    let pull_free: libloading::Symbol<unsafe extern "C" fn(PullResult)> =
      lib.get(b"pull_free").unwrap();
    let res = pull_run();
    if res.is_ok {
      println!("data: {:?}", res.get_data_json())
    } else {
      println!("error: {:?}", res.get_error_message())
    }
    pull_free(res);
  }
}
