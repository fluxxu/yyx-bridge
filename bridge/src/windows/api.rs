#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use bridge_derive::secret_string_from_file;
use lazy_static::lazy_static;
use std::ffi::CString;
use std::mem::transmute;
use std::os::raw::c_char;
use std::sync::Mutex;

use super::process;
use super::result::*;

const OFFSET_PYGILSTATE_ENSURE: isize = 0xc139e0;
const OFFSET_PYGILSTATE_RELEASE: isize = 0xc13a70;
const OFFSET_PYRUN_SIMPLESTRINGFLAGS: isize = 0xc0dc90;

type _PyGILState_Ensure = unsafe extern "cdecl" fn() -> *const ();
type _PyGILState_Release = unsafe extern "cdecl" fn(*const ());
type PyRun_SimpleStringFlags = unsafe extern "cdecl" fn(*const c_char, i32) -> i32;

lazy_static! {
  static ref STATE: Mutex<Option<State>> = { Mutex::new(None) };
}

struct State {
  p_PyGILState_Ensure: _PyGILState_Ensure,
  p_PyGILState_Release: _PyGILState_Release,
  p_PyRun_SimpleStringFlags: PyRun_SimpleStringFlags,
}

#[inline(always)]
pub fn init() -> BridgeResult<()> {
  let version = process::get_version_string();
  if version != secret_string_from_file!("bridge/assets/supported_version.txt") {
    return Err(BridgeError::VersionNotSupported(version));
  }
  let code_section = process::get_code_section().ok_or_else(|| BridgeError::GetBase)?;
  let base = code_section.base;
  let value = unsafe {
    State {
      p_PyGILState_Ensure: transmute(base.offset(OFFSET_PYGILSTATE_ENSURE)),
      p_PyGILState_Release: transmute(base.offset(OFFSET_PYGILSTATE_RELEASE)),
      p_PyRun_SimpleStringFlags: transmute(base.offset(OFFSET_PYRUN_SIMPLESTRINGFLAGS)),
    }
  };
  let mut state = STATE.lock().unwrap();
  *state = Some(value);
  Ok(())
}

#[inline(always)]
pub fn run(code: &str) -> BridgeResult<i32> {
  debug!("running code: {}", code);
  let state = STATE.lock().unwrap();
  if let Some(state) = state.as_ref() {
    unsafe {
      let gil = (state.p_PyGILState_Ensure)();
      let code_cstr = CString::new(code).unwrap();
      let rv = (state.p_PyRun_SimpleStringFlags)(code_cstr.as_ptr(), 0);
      (state.p_PyGILState_Release)(gil);
      debug!("run result = {}", rv);
      Ok(rv)
    }
  } else {
    Err(BridgeError::Internal.into())
  }
}
