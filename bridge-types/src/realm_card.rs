use bridge_derive::ParseClientValue;
use serde_derive::Serialize;

#[derive(Debug, Serialize, ParseClientValue, PartialEq)]
pub struct RealmCard {
  pub id: String,
  pub item_id: i64,
  pub total_time: i64,
  pub attrs: RealmCardAttr,
}

#[derive(Debug, Serialize, ParseClientValue, PartialEq)]
pub struct RealmCardAttr {
  pub exp: i64,
  pub bonus: i64,
}
