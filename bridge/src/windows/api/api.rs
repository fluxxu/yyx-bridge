#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use bridge_derive::secret_string;
use lazy_static::lazy_static;
use std::ffi::CString;
use std::mem::transmute;
use std::os::raw::c_char;
use std::sync::Mutex;

use crate::windows::byte_pattern;
use crate::windows::process;
use crate::windows::result::*;

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
fn resolve_state(code_section: &process::Section) -> Option<State> {
  let P_PYGILSTATE_ENSURE = secret_string!("56 57 FF 35 ?? ?? ?? ?? E8 ?? ?? ?? ?? 8B F0 83 C4 04");
  let p_PyGILState_Ensure = {
    let pattern = byte_pattern::PatternFinder::new(&P_PYGILSTATE_ENSURE);
    pattern.find_pattern(code_section.base, code_section.size)?
  };

  let P_PYGILSTATE_RELEASE = secret_string!("55 8B EC 56 FF 35 ?? ?? ?? ?? E8 ?? ?? ?? ?? 8B F0");
  let p_PyGILState_Release = {
    let pattern = byte_pattern::PatternFinder::new(&P_PYGILSTATE_RELEASE);
    pattern.find_pattern(code_section.base, code_section.size)?
  };

  let P_PYRUN_SIMPLESTRINGFLAGS =
    secret_string!("55 8B EC 68 ?? ?? ?? ?? E8 ?? ?? ?? ?? 83 C4 04 85 C0 74 26");
  let p_PyRun_SimpleStringFlags = {
    let pattern = byte_pattern::PatternFinder::new(&P_PYRUN_SIMPLESTRINGFLAGS);
    pattern.find_pattern(code_section.base, code_section.size)?
  };

  unsafe {
    Some(State {
      p_PyGILState_Ensure: transmute(p_PyGILState_Ensure),
      p_PyGILState_Release: transmute(p_PyGILState_Release),
      p_PyRun_SimpleStringFlags: transmute(p_PyRun_SimpleStringFlags),
    })
  }
}

#[inline(always)]
pub fn init() -> BridgeResult<()> {
  let version = process::get_version_string();

  debug!("client version = {}", version);

  let code_section = process::get_code_section().ok_or_else(|| BridgeError::GetBase)?;
  let value =
    resolve_state(&code_section).ok_or_else(|| BridgeError::VersionNotSupported(version))?;
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
