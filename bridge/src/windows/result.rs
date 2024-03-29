use super::inject::InjectError;

#[derive(Debug, Fail)]
pub enum BridgeError {
  #[fail(display = "Install error: {:?}", _0)]
  Inject(InjectError),
  #[fail(display = "Get base error.")]
  GetBase,
  #[fail(display = "Unsupported client version: {}", _0)]
  VersionNotSupported(String),
  #[fail(display = "Unknown internal error.")]
  Internal
}

pub type BridgeResult<T> = Result<T, BridgeError>;