use bridge::PullResult;
use chrono::Local;
use std::io::prelude::*;

mod macos;

fn main() {
  use bridge::*;
  use std::ffi::CStr;

  unsafe {
    let version = version_get();
    let version_str = CStr::from_ptr(version).to_string_lossy().to_string();
    version_free(version);

    println!("Bridge version: {}", version_str);
    println!("Generating snapshot...");

    let res = pull_emulator();
    handle_result(&version_str, &res);
    pull_free(res);
  }
}

fn handle_result(version_str: &str, res: &PullResult) {
  use std::fs::write;
  use std::io::stdin;

  let self_dir = macos::get_self_dir();
  println!("Output dir: {:?}", self_dir);

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
    let path = format!("yyx_snapshot_{}_{}.json", ts, short_id);
    write(
      self_dir.join(&path),
      serde_json::to_string_pretty(&json!({
        "timestamp": now,
        "version": version_str,
        "data": value
      }))
      .unwrap(),
    )
    .unwrap();
    println!("Snapshot generated: {}", path);
  } else {
    println!(
      "Error: {}",
      res
        .get_error_message()
        .unwrap_or_else(|| "Unknown error.".to_string())
    );
    if let Some(data) = res.get_data_json() {
      write(self_dir.join("last_error_data.json"), data).unwrap();
    }
    for _ in stdin().lock().bytes() {
      break;
    }
  }
}
