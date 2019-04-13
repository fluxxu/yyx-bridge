mod result;

#[cfg(not(feature = "guild"))]
use bridge_types::Snapshot;

#[cfg(feature = "guild")]
use bridge_types::Guild as Snapshot;

use crate::deserialize::{deserialize_data, DeserializeError};
use crate::result::PullResult;

use self::result::*;
use adb_rs::AdbClient;
use std::io;

pub fn run() -> PullResult {
  #[cfg(debug_assertions)]
  {
    use simplelog::*;

    CombinedLogger::init(vec![]).ok();
  }

  match run_impl() {
    Ok(data) => PullResult::ok(data),
    Err(err) => {
      return PullResult::err(&err.to_string());
    }
  }
}

#[cfg(not(feature = "guild"))]
const BRIDGE_BINARY: &[u8] = include_bytes!("../../assets/yyx-bridge-android") as &[u8];

#[cfg(feature = "guild")]
const BRIDGE_BINARY: &[u8] = include_bytes!("../../assets/yyx-bridge-guild-android") as &[u8];

pub fn run_impl() -> BridgeResult<Snapshot> {
  use adb_rs::push::AdbPush;
  use adb_rs::shell::AdbShell;
  use std::io::Cursor;

  println!("Initializing...");
  match kill_adb_server() {
    Ok(_) => {}
    Err(_err) => debug!("err: {}", _err),
  }

  println!("Connecting to MuMu...");
  #[cfg(target_os = "windows")]
  let mut conn = AdbClient::new("host::").connect("127.0.0.1:7555")?;
  #[cfg(target_os = "macos")]
  let mut conn = AdbClient::new("host::").connect("127.0.0.1:5555")?;

  let bytes = Cursor::new(BRIDGE_BINARY);

  println!("Pushing YYX bridge...");
  conn.push_reader(bytes, "/vendor/yyx-bridge")?;

  println!("Running YYX bridge...");
  let data = conn
    .shell_exec("chmod +x /vendor/yyx-bridge && /vendor/yyx-bridge && rm -f /vendor/yyx-bridge")?;

  println!("Parsing data...");
  match deserialize_data(&data) {
    Ok(data) => Ok(data),
    Err(DeserializeError::ParseSnapshotData(msg)) => Err(BridgeError::Msg(msg)),
  }
}

fn kill_adb_server() -> Result<(), io::Error> {
  use std::io::prelude::*;
  use std::net::TcpStream;
  let mut stream = TcpStream::connect("127.0.0.1:5037")?;
  stream.write_all(b"0009host:kill")?;
  let mut buf = [0; 4096];
  loop {
    let n = stream.read(&mut buf)?;
    if n == 0 {
      break;
    }
  }
  Ok(())
}

#[test]
fn test_run() {
  run();
}
