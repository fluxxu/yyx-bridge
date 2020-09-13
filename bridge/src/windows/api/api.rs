#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use bridge_derive::secret_string;
use lazy_static::lazy_static;
use std::ffi::CString;
use std::mem::transmute;
use std::os::raw::c_char;
use std::sync::Mutex;
use std::ptr;

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
pub fn init() -> BridgeResult<()> {
  use winapi::um::libloaderapi::*;
  unsafe {
    let m = GetModuleHandleA(ptr::null_mut());
    if m == ptr::null_mut() {
      return Err(BridgeError::Internal);
    }

    let procname = CString::new(secret_string!("PyGILState_Ensure")).unwrap();
    let ensure = GetProcAddress(m, procname.as_ptr());

    let procname = CString::new(secret_string!("PyGILState_Release")).unwrap();
    let release = GetProcAddress(m, procname.as_ptr());

    let procname = CString::new(secret_string!("PyRun_SimpleStringFlags")).unwrap();
    let run = GetProcAddress(m, procname.as_ptr());

    if ensure == ptr::null_mut() || release == ptr::null_mut() || run == ptr::null_mut() {
      return Err(BridgeError::Internal);
    }

    let mut state = STATE.lock().unwrap();
    *state = Some(State {
      p_PyGILState_Ensure: transmute(ensure),
      p_PyGILState_Release: transmute(release),
      p_PyRun_SimpleStringFlags: transmute(run),
    });
  }

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
