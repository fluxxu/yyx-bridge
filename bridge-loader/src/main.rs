use std::io::{self, Read};
use std::os::raw::c_char;
use std::ptr;
use chrono::{Local};

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
  use std::fs::write;
  use std::ffi::CStr;

  println!("Loading...");

  let lib = libloading::Library::new("bridge.dll").unwrap();
  unsafe {
    let pull_run: libloading::Symbol<unsafe extern "C" fn() -> PullResult> =
      lib.get(b"pull_run").unwrap();
    let pull_free: libloading::Symbol<unsafe extern "C" fn(PullResult)> =
      lib.get(b"pull_free").unwrap();
    let version_get: libloading::Symbol<unsafe extern "C" fn() -> *mut c_char> =
      lib.get(b"version_get").unwrap();
    let version_free: libloading::Symbol<unsafe extern "C" fn(*mut c_char)> =
      lib.get(b"version_free").unwrap();

    let version = version_get();
    let version_str = CStr::from_ptr(version).to_string_lossy().to_string();
    version_free(version);

    println!("Bridge version: {}", version_str);
    println!("Generating snapshot...");

    let res = pull_run();
    if res.is_ok {
      use serde_json::{self, json};
      let now = Local::now();
      let ts = Local::now().format("%Y%m%d%H%M%S");

      let value: serde_json::Value = serde_json::from_str(&res.get_data_json().unwrap()).unwrap();
      write(
        format!("yyx_snapshot_{}.json", ts),
        serde_json::to_string_pretty(&json!({
          "timestamp": now,
          "version": version_str,
          "data": value
        })).unwrap(),
      )
      .unwrap();
    } else {
      println!("Error: {}", res.get_error_message().unwrap_or_else(|| "Unknown error.".to_string()));
      if let Some(data) = res.get_data_json() {
        write("last_error_data.json", data).unwrap();
      }
      for _ in io::stdin().lock().bytes() {
        break;
      }
    }
    pull_free(res);
  }
}
