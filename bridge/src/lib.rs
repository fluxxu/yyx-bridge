#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate log;

pub mod inject;
pub mod process;
pub mod ptr;
pub mod result;
pub mod utils;
pub mod window;

mod pull;

#[no_mangle]
pub extern "stdcall" fn DllMain(
  hinst_dll: winapi::shared::minwindef::HINSTANCE,
  fdw_reason: u32,
  _: *mut winapi::ctypes::c_void,
) {
  match fdw_reason {
    1 => {
      #[cfg(debug_assertions)]
      {
        use simplelog::*;
        use std::fs::File;
        CombinedLogger::init(vec![
          TermLogger::new(LevelFilter::Debug, Config::default()).unwrap(),
          // WriteLogger::new(
          //   LevelFilter::Debug,
          //   Config::default(),
          //   File::create("PandaOnmyoji.Bridge.log").unwrap(),
          // ),
        ])
        .ok();
      }
      debug!("bridge loaded.");
      debug!(
        "process = {:?}",
        utils::get_module_path(::std::ptr::null_mut())
      );
      debug!("self = {:?}", utils::get_module_path(hinst_dll));
      debug!("version = {}", process::get_version_string());
    }
    0 => {
      debug!("bridge module unloaded.");
    }
    other => debug!("bridge dllmain: {}", other),
  }
}

#[no_mangle]
pub unsafe extern "C" fn pull_run() -> pull::PullResult {
  pull::run_server()
}

#[no_mangle]
pub unsafe extern "C" fn pull_free(result: pull::PullResult) {
  pull::free_result(result)
}
