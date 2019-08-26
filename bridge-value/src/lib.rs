pub use serde_json::Value;

#[derive(Debug)]
pub enum ParseClientValueError {
  TypeMismatch(&'static str, Value),
  Message(String),
}

pub trait ParseClientValue: Sized {
  fn parse_client_value(value: &Value) -> Result<Self, ParseClientValueError>;
}

impl<T> ParseClientValue for Vec<T>
where
  T: ParseClientValue,
{
  fn parse_client_value(value: &Value) -> Result<Self, ParseClientValueError> {
    if let Some(vec) = value.as_array() {
      use std::iter::FromIterator;
      let iter = vec
        .into_iter()
        .enumerate()
        .map(|(i, v)| T::parse_client_value(&v).map_err(|err| (i, err, v.clone())));
      Result::from_iter(iter).map_err(|(i, err, v)| {
        ParseClientValueError::Message(format!("error at element {}: {:?}: {}", i, err, v))
      })
    } else {
      Err(ParseClientValueError::TypeMismatch("Vec<T>", value.clone()))
    }
  }
}

impl<T> ParseClientValue for Option<T>
where
  T: ParseClientValue,
{
  fn parse_client_value(value: &Value) -> Result<Self, ParseClientValueError> {
    if value.is_null() {
      Ok(None)
    } else {
      Ok(Some(T::parse_client_value(value)?))
    }
  }
}

impl ParseClientValue for String {
  fn parse_client_value(value: &Value) -> Result<Self, ParseClientValueError> {
    value
      .as_str()
      .map(|v| v.to_owned())
      .ok_or_else(|| ParseClientValueError::TypeMismatch("String", value.clone()))
  }
}

impl ParseClientValue for i64 {
  fn parse_client_value(value: &Value) -> Result<Self, ParseClientValueError> {
    value
      .as_i64()
      .ok_or_else(|| ParseClientValueError::TypeMismatch("i64", value.clone()))
  }
}

impl ParseClientValue for u64 {
  fn parse_client_value(value: &Value) -> Result<Self, ParseClientValueError> {
    value
      .as_u64()
      .ok_or_else(|| ParseClientValueError::TypeMismatch("u64", value.clone()))
  }
}

impl ParseClientValue for f64 {
  fn parse_client_value(value: &Value) -> Result<Self, ParseClientValueError> {
    value
      .as_f64()
      .ok_or_else(|| ParseClientValueError::TypeMismatch("f64", value.clone()))
  }
}

impl ParseClientValue for bool {
  fn parse_client_value(value: &Value) -> Result<Self, ParseClientValueError> {
    value
      .as_bool()
      .ok_or_else(|| ParseClientValueError::TypeMismatch("bool", value.clone()))
  }
}
