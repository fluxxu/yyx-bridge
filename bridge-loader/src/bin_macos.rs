mod macos;
mod lib_macos;

fn main() {
  use std::ffi::CString;
  let self_dir = macos::get_self_dir();
  println!("Output dir: {:?}", self_dir);

  let cstr = CString::new(self_dir.to_string_lossy().as_bytes()).unwrap();
  lib_macos::run(cstr.as_ptr());
}