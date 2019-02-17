use crate::inject::InjectError;

#[derive(Debug, Fail)]
pub enum UtilsError {
  #[fail(display = "inject error: {:?}", _0)]
  Inject(InjectError)
}

pub type UtilsResult<T> = Result<T, UtilsError>;