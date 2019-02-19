use crate::inject::InjectError;

#[derive(Debug, Fail)]
pub enum BridgeError {
  #[fail(display = "install error: {:?}", _0)]
  Inject(InjectError),
  #[fail(display = "get base error.")]
  GetBase,
  #[fail(display = "unsupported client version: {}", _0)]
  VersionNotSupported(String),
  #[fail(display = "unknown internal error.")]
  Internel,
}

pub type BridgeResult<T> = Result<T, BridgeError>;
