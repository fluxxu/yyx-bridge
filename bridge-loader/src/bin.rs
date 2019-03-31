#[cfg(target_os = "macos")]
include!("./bin_macos.rs");

#[cfg(not(target_os = "macos"))]
fn main() {}
