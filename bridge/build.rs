extern crate cc;

fn main() {
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
}