#[cfg(target_os = "windows")]
extern crate cc;

#[cfg(target_os = "windows")]
fn main() {
  println!("cargo:rerun-if-changed=assets/client.py");
  println!("cargo:rustc-link-lib=dylib=user32");

  cc::Build::new()
    .cpp(true)
    .file("src/windows/window.cpp")
    .compile("window");

  cc::Build::new()
    .cpp(true)
    .file("src/windows/inject.cpp")
    .compile("inject");

  cc::Build::new()
    .cpp(true)
    .file("src/windows/process.cpp")
    .compile("process");

  cc::Build::new()
    .cpp(true)
    .file("src/windows/byte_pattern.cpp")
    .compile("byte_pattern");

  write_version()
}

#[cfg(target_os = "macos")]
fn main() {
  println!("cargo:rerun-if-changed=assets/yyx-bridge-android");
  println!("cargo:rerun-if-changed=assets/yyx-bridge-guild-android");
  write_version()
}

fn write_version() {
  use std::{fs, env};
  let verison_file_path = format!("{}/version.rs", env::var("OUT_DIR").unwrap());
  let version = env::var("CARGO_PKG_VERSION").unwrap();
  fs::write(verison_file_path, format!(r#"pub const VERSION: &str = "{}";"#, version)).unwrap();
}