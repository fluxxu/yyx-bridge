#[cfg(target_os = "window")]
include!("./bin_windows.rs");

#[cfg(target_os = "macos")]
include!("./bin_macos.rs");