use bridge_derive::ParseClientValue;
use serde_derive::Serialize;

#[derive(Debug, Serialize, ParseClientValue, PartialEq)]
pub struct HeroBookShard {
  pub hero_id: i64,
  pub shards: i64,
  pub books: i64,
  pub book_max_shards: i64,
}
