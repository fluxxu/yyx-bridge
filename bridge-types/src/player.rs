use bridge_derive::ParseClientValue;
use serde_derive::Serialize;

#[derive(Debug, Serialize, ParseClientValue, PartialEq)]
pub struct Player {
  pub id: i64,
  pub name: String,
  pub level: i64,
}

#[derive(Debug, Serialize, ParseClientValue, PartialEq)]
pub struct PlayerCurrency {
  pub coin: i64,
  pub jade: i64,
  pub action_point: i64,
}