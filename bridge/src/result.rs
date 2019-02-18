use crate::inject::InjectError;

#[derive(Debug, Fail)]
pub enum BridgeError {
  #[fail(display = "install error: {:?}", _0)]
  Inject(InjectError),
}

pub type BridgeResult<T> = Result<T, BridgeError>;
