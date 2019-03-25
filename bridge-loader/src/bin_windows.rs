use chrono::Local;
use libloading::{self, Library, Symbol};
use std::io::prelude::*;
use std::os::raw::c_char;
use std::ptr;

struct LibInterface<'a> {
  pull_windows: Symbol<'a, extern "C" fn() -> PullResult>,
  pull_emulator: Symbol<'a, extern "C" fn() -> PullResult>,
  pull_free: Symbol<'a, extern "C" fn(PullResult)>,
  version_get: Symbol<'a, extern "C" fn() -> *mut c_char>,
  version_free: Symbol<'a, extern "C" fn(*mut c_char)>,
}

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

#[cfg(target_os = "windows")]
fn main() {
  use std::ffi::CStr;

  println!("Loading...");

  let lib = libloading::Library::new("bridge.dll").unwrap();
  let LibInterface {
    pull_windows,
    pull_free,
    version_get,
    version_free,
  } = get_symbols(&lib);
  let version = version_get();
  let version_str = unsafe { CStr::from_ptr(version).to_string_lossy().to_string() };
  version_free(version);

  println!("Bridge version: {}", version_str);
  println!("Generating snapshot...");

  let res = pull_windows();
  handle_result(&version_str, &res);
  pull_free(res);
}

#[cfg(target_os = "windows")]
fn get_symbols<'a>(lib: &'a Library) -> LibInterface<'a> {
  unsafe {
    LibInterface {
      pull_windows: lib.get(b"pull_windows").unwrap(),
      pull_emulator: lib.get(b"pull_emulator").unwrap(),
      pull_free: lib.get(b"pull_free").unwrap(),
      version_get: lib.get(b"version_get").unwrap(),
      version_free: lib.get(b"version_free").unwrap(),
    }
  }
}

fn handle_result(version_str: &str, res: &PullResult) {
  use std::fs::write;
  use std::io::stdin;

  if res.is_ok {
    use serde_json::{self, json};
    let now = Local::now();
    let ts = Local::now().format("%Y%m%d%H%M%S");

    let value: serde_json::Value = serde_json::from_str(&res.get_data_json().unwrap()).unwrap();
    let short_id: i64 = value
      .as_object()
      .and_then(|obj| {
        obj.get("player").and_then(|p| {
          p.as_object()
            .and_then(|p| p.get("id").and_then(|id| id.as_i64()))
        })
      })
      .unwrap_or(0);
    write(
      format!("yyx_snapshot_{}_{}.json", ts, short_id),
      serde_json::to_string_pretty(&json!({
        "timestamp": now,
        "version": version_str,
        "data": value
      }))
      .unwrap(),
    )
    .unwrap();
  } else {
    println!(
      "Error: {}",
      res
        .get_error_message()
        .unwrap_or_else(|| "Unknown error.".to_string())
    );
    if let Some(data) = res.get_data_json() {
      write("last_error_data.json", data).unwrap();
    }
    for _ in stdin().lock().bytes() {
      break;
    }
  }
}
