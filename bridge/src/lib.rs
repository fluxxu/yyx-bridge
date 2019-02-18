#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate crossbeam_channel;

#[cfg(not(debug_assertions))]
macro_rules! debug {
  ($($tt:tt),*) => {
    ()
  };
}

mod inject;
mod process;
mod ptr;
mod result;
mod utils;
mod window;

mod pull;

pub use self::pull::PullResult;

use lazy_static::lazy_static;
use std::path::PathBuf;
use std::sync::Mutex;
use winapi::shared::minwindef::HINSTANCE;
use bridge_derive::secret_string;

#[derive(Debug, Clone)]
pub struct Env {
  pub self_path: PathBuf,
}

lazy_static! {
  pub static ref ENV: Mutex<Option<Env>> = { Mutex::new(None) };
}

#[no_mangle]
pub extern "stdcall" fn DllMain(
  hinst_dll: HINSTANCE,
  fdw_reason: u32,
  _: *mut winapi::ctypes::c_void,
) {
  match fdw_reason {
    1 => run(hinst_dll),
    0 => {
      debug!("bridge module unloaded.");
    }
    other => debug!("bridge dllmain: {}", other),
  }
}

#[cfg(debug_assertions)]
fn run(hinst_dll: HINSTANCE) {
  use simplelog::*;
  use std::fs::File;
  let env = init_env(hinst_dll);
  if is_game_process() {
    CombinedLogger::init(vec![WriteLogger::new(
      LevelFilter::Debug,
      Config::default(),
      File::create(env.self_path.with_file_name("bridge.log")).unwrap(),
    )])
    .ok();
    pull::run_client();
  } else {
    CombinedLogger::init(vec![
      TermLogger::new(LevelFilter::Debug, Config::default()).unwrap()
    ])
    .ok();
  }
}

#[cfg(not(debug_assertions))]
fn run(hinst_dll: HINSTANCE) {
  use simplelog::*;
  use std::fs::File;
  let env = init_env(hinst_dll);
  if is_game_process() {
    CombinedLogger::init(vec![WriteLogger::new(
      LevelFilter::Debug,
      Config::default(),
      File::create(env.self_path.with_file_name("bridge.log")).unwrap(),
    )])
    .ok();
    pull::run_client();
  }
}

fn init_env(hinst_dll: HINSTANCE) -> Env {
  let self_path = utils::get_module_path(hinst_dll);
  let value = Env { self_path };
  let mut r = ENV.lock().unwrap();
  *r = Some(value.clone());
  value
}

fn is_game_process() -> bool {
  let app_path = utils::get_module_path(::std::ptr::null_mut());
  if let Some(name) = app_path.file_name().and_then(|v| v.to_str()) {
    return name == secret_string!("onmyoji.exe")
  } else {
    return false
  }
}

pub fn get_env() -> Option<Env> {
  let r = ENV.lock().unwrap();
  r.clone()
}

#[no_mangle]
pub unsafe extern "C" fn pull_run() -> pull::PullResult {
  pull::run_server()
}

#[no_mangle]
pub unsafe extern "C" fn pull_free(result: pull::PullResult) {
  pull::free_result(result)
}
