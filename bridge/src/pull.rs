use serde::Serialize;
use std::ffi::CString;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr;
use std::thread;
use crossbeam_channel::{bounded, Sender, Receiver, TryRecvError};
use winapi::shared::ntdef::HANDLE;

pub struct PullResult {
  pub is_ok: bool,
  pub error_message: *mut c_char,
  pub data_json: *mut c_char,
}

impl PullResult {
  pub fn ok<T: Serialize>(data: T) -> Self {
    use serde_json;
    let json = serde_json::to_string(&data).unwrap();
    PullResult {
      is_ok: true,
      error_message: ptr::null_mut(),
      data_json: CString::new(json).unwrap().into_raw(),
    }
  }

  pub fn err(message: &str) -> Self {
    PullResult {
      is_ok: false,
      error_message: CString::new(message).unwrap().into_raw(),
      data_json: ptr::null_mut(),
    }
  }
}

pub fn run_client() {
  use crate::process;
  let version = process::get_version_string();
  debug!("version = {}", version);
  ::std::thread::sleep_ms(3000);
  unsafe {
    // ::winapi::um::processthreadsapi::TerminateProcess(::winapi::um::processthreadsapi::GetCurrentProcess(), 0);
  }
}

pub fn run_server() -> PullResult {
  use crate::{get_env, inject};
  use bridge_derive::secret_string;

  let env = get_env().unwrap();
  let dll_path = env.self_path;

  let (cmd_s, cmd_r) = bounded(1);
  let (rep_s, rep_r) = bounded(1);

  let worker = thread::spawn(move || {
    pipe_server_worker(rep_s, cmd_r)
  });

  match rep_r.recv().unwrap() {
    PipeMsg::ServerStarted => {},
    PipeMsg::ServerError(err) => {
      return PullResult::err(&format!("Worker error: {}", err))
    },
    _ => {
      return PullResult::err(&format!("Unexpected message type."))
    },
  };


  debug!("pipe server started.");

  // debug!("pipe server started, injecting {:?}...", dll_path);
  // let remote_err = if let Err(err) = inject::inject_dll_to_yys(dll_path) {
  //   Some(err.to_string())
  // } else {
  //   None
  // };

  // cmd_s.send(PipeMsg::CmdTerm).unwrap();

  // unsafe {
  //   use winapi::shared::winerror::*;
  //   use winapi::um::errhandlingapi::*;
  //   use std::os::windows::io::AsRawHandle;
  //   loop {
  //     let r = ::winapi::um::ioapiset::CancelSynchronousIo(worker.as_raw_handle());
  //     if r == 1 {
  //       break
  //     }
  //     let last_err = GetLastError();
  //     if r != 1 && last_err == ERROR_NOT_FOUND {
  //       thread::sleep(::std::time::Duration::from_millis(200));
  //     } else {
  //       panic!("Unknown worker error: {}", last_err);
  //     }
  //   }
  // }

  match rep_r.recv().unwrap() {
    PipeMsg::ServerStopped {
      err,
      data
    }=> {
      debug!("pipe server stopped.");
      if let Some(err) = err {
        PullResult::err(&err)
      } else {
        match String::from_utf8(data) {
          Ok(data) => PullResult::ok(data),
          Err(err) => PullResult::err(&format!("Invalid utf-8 bytes."))
        }
      }
    },
    _ => PullResult::err(&format!("Unexpected message type."))
  }
}

enum PipeMsg {
  ServerStarted,
  ServerError(String),
  ServerStopped {
    err: Option<String>,
    data: Vec<u8>,
  },
  CmdTerm,
}

fn pipe_server_worker(s: Sender<PipeMsg>, r: Receiver<PipeMsg>) {
  use bridge_derive::secret_string;
  let pipe_path = secret_string!(r#"\\.\pipe\b62340b3-9f87-4f38-b844-7b8d1598b64b"#);

  enum ErrorCode {
    CreatePipe = 1,
  }

  unsafe {
    use std::ffi::CString;
    use std::ptr;
    use winapi::shared::winerror::*;
    use winapi::um::errhandlingapi::*;
    use winapi::um::fileapi::*;
    use winapi::um::handleapi::*;
    use winapi::um::namedpipeapi::*;
    use winapi::um::winbase::*;

    const BUFFER_SIZE: u32 = 1024 * 1024;

    let pipe = CreateNamedPipeA(
      CString::new(pipe_path).unwrap().as_ptr(),
      PIPE_ACCESS_DUPLEX,
      PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
      1,
      BUFFER_SIZE,
      BUFFER_SIZE,
      0,
      ptr::null_mut(),
    );

    if pipe == INVALID_HANDLE_VALUE {
      s.send(PipeMsg::ServerError(
        format!("{},{}", ErrorCode::CreatePipe as i32, GetLastError())
      )).unwrap();
      return
    }

    s.send(PipeMsg::ServerStarted).unwrap();


    let connected =
      ConnectNamedPipe(pipe, ptr::null_mut()) == 1 || GetLastError() == ERROR_PIPE_CONNECTED;

    let mut terminated = !connected || match r.try_recv() {
      Ok(PipeMsg::CmdTerm) => {
        true
      },
      Err(TryRecvError::Empty) => false,
      Err(TryRecvError::Disconnected) => {
        true
      },
      _ => false,
    };

    let mut data = vec![];
    let mut last_err = 0;
    if !terminated {
      loop {
        let mut buf: Vec<u8> = Vec::with_capacity(BUFFER_SIZE as usize);
        buf.resize(BUFFER_SIZE as usize, 0);
        let mut bytes_read: u32 = 0;
        let ok = ReadFile(
          pipe,
          buf.as_mut_ptr() as *mut winapi::ctypes::c_void,
          BUFFER_SIZE,
          &mut bytes_read as *mut u32,
          ptr::null_mut(),
        );
        if ok != 1 || bytes_read == 0 {
          match GetLastError() {
            ERROR_BROKEN_PIPE => {
              debug!("client disconnected.");
              DisconnectNamedPipe(pipe);
              break;
            },
            ERROR_MORE_DATA => {},
            code => {
              last_err = code;
              terminated = true;
              debug!("ReadFile error: {}", last_err);
              break;
            }
          }
        }

        debug!("bytes read = {}", bytes_read);
        data.extend(&buf[..(bytes_read as usize)]);
      }
    }

    CloseHandle(pipe);
    s.send(PipeMsg::ServerStopped {
      err: if terminated { Some(format!("Terminated: {}", last_err)) } else { None },
      data
    }).unwrap();
  }
}

pub unsafe fn free_result(result: PullResult) {
  if result.error_message != ptr::null_mut() {
    CString::from_raw(result.error_message);
  }
  if result.data_json != ptr::null_mut() {
    CString::from_raw(result.data_json);
  }
}
