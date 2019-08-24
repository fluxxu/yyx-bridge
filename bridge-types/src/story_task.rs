use bridge_derive::ParseClientValue;
use serde_derive::Serialize;

#[derive(Debug, Serialize, ParseClientValue, PartialEq)]
pub struct StoryTask {
  pub id: i64,
  pub progress: StoryTaskProgress,
}

#[derive(Debug, Serialize, ParseClientValue, PartialEq)]
pub struct StoryTaskProgress {
  pub value: i64,
  pub max_value: i64,
}