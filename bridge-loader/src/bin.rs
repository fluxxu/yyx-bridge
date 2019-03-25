#[cfg(target_os = "windows")]
include!("./bin_windows.rs");

#[cfg(target_os = "macos")]
include!("./bin_macos.rs");
