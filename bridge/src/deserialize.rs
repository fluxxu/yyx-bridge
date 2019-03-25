use bridge_types::Snapshot;
use bridge_value;
use serde_json::Value;

pub enum DeserializeError {
  ParseSnapshotData(String),
}

pub fn deserialize_data(bytes: &[u8]) -> Result<Snapshot, DeserializeError> {
  use bridge_value::{ParseClientValue, ParseClientValueError};
  let value: Value = serde_json::from_reader(bytes)
    .map_err(|err| DeserializeError::ParseSnapshotData(err.to_string()))?;
  if let Some(msg) = value
    .as_object()
    .and_then(|o| o.get("error").cloned())
    .and_then(|v| v.as_str().map(|v| v.to_owned()))
  {
    return Err(DeserializeError::ParseSnapshotData(msg.to_owned()));
  }
  Snapshot::parse_client_value(&value).map_err(|err| {
    DeserializeError::ParseSnapshotData(match err {
      ParseClientValueError::TypeMismatch => format!("Type mismatch."),
      ParseClientValueError::Message(msg) => msg,
    })
  })
}
