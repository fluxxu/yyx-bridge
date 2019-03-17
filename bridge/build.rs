#[cfg(target_os = "windows")]
extern crate cc;

#[cfg(target_os = "windows")]
fn main() {
  println!("cargo:rerun-if-changed=assets/client.py");
  println!("cargo:rustc-link-lib=dylib=user32");

  cc::Build::new()
    .cpp(true)
    .file("src/window.cpp")
    .compile("window");

  cc::Build::new()
    .cpp(true)
    .file("src/inject.cpp")
    .compile("inject");

  cc::Build::new()
    .cpp(true)
    .file("src/process.cpp")
    .compile("process");

  write_version()
}

#[cfg(not(target_os = "windows"))]
fn main() {
  write_version()
}

fn write_version() {
  use std::{fs, env};
  let verison_file_path = format!("{}/version.rs", env::var("OUT_DIR").unwrap());
  let version = env::var("CARGO_PKG_VERSION").unwrap();
  fs::write(verison_file_path, format!(r#"pub const VERSION: &str = "{}";"#, version)).unwrap();
}