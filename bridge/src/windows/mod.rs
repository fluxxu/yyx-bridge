mod result;

mod byte_pattern;
mod inject;
mod process;
mod ptr;
mod utils;

#[cfg(not(feature = "fg"))]
mod window;

mod api;
mod pull;

use bridge_derive::secret_string;
use lazy_static::lazy_static;
use std::path::PathBuf;
use std::sync::Mutex;
use winapi::shared::minwindef::HINSTANCE;

#[derive(Debug, Clone)]
pub struct Env {
  pub self_path: PathBuf,
}

lazy_static! {
  pub static ref ENV: Mutex<Option<Env>> = { Mutex::new(None) };
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
  debug!("app path = {:?}", app_path);
  if let Some(name) = app_path.file_name().and_then(|v| v.to_str()) {
    let name_low = name.to_lowercase();
    return name_low == secret_string!("onmyoji.exe") || name_low == secret_string!("client.exe");
  } else {
    return false;
  }
}

fn get_env() -> Option<Env> {
  let r = ENV.lock().unwrap();
  r.clone()
}

#[cfg(debug_assertions)]
pub fn run(hinst_dll: HINSTANCE) {
  use simplelog::*;
  use std::fs::File;
  let env = init_env(hinst_dll);

  CombinedLogger::init(vec![WriteLogger::new(
    LevelFilter::Debug,
    Config::default(),
    File::create(env.self_path.with_file_name("bridge.log")).unwrap(),
  )])
  .ok();

  if is_game_process() {
    pull::run_client();
  }
}

#[cfg(not(debug_assertions))]
pub fn run(hinst_dll: HINSTANCE) {
  init_env(hinst_dll);
  if is_game_process() {
    pull::run_client();
  }
}

pub use self::pull::run_server;
