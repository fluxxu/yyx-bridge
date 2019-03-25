#[derive(Debug, Fail)]
pub enum BridgeError {
  #[fail(display = "IO error: {:?}", _0)]
  Io(::std::io::Error),
  #[fail(display = "Adb error: {:?}", _0)]
  Adb(::adb_rs::result::AdbError),
  #[fail(display = "{}", _0)]
  Msg(String),
}

impl From<::std::io::Error> for BridgeError {
  fn from(v: ::std::io::Error) -> BridgeError {
    BridgeError::Io(v)
  }
}

impl From<::adb_rs::result::AdbError> for BridgeError {
  fn from(v: ::adb_rs::result::AdbError) -> BridgeError {
    BridgeError::Adb(v)
  }
}

pub type BridgeResult<T> = Result<T, BridgeError>;
