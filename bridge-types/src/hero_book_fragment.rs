use bridge_derive::ParseClientValue;
use serde_derive::Serialize;

#[derive(Debug, Serialize, ParseClientValue, PartialEq)]
pub struct HeroBookFragment {
  pub hero_id: i64,
  pub fragments: i64,
  pub books: i64,
  pub book_fragments: i64,
}
