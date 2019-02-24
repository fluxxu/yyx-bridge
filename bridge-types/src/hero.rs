use bridge_derive::ParseClientValue;
use serde_derive::Serialize;

#[derive(Debug, Serialize, ParseClientValue, PartialEq)]
pub struct Hero {
  pub id: String,
  pub hero_id: i64,
  pub equips: Vec<String>,
  pub level: i64,
  pub exp: f64,
  pub exp_rate: f64,
  pub nick_name: String,
  pub born: i64,
  pub lock: bool,
  pub rarity: HeroRarity,
  pub skills: Vec<HeroSkill>,
}

#[derive(Debug, Serialize, ParseClientValue, PartialEq)]
pub struct HeroSkill {
  pub id: i64,
  pub level: i64,
}

#[derive(Debug, Serialize, ParseClientValue, Copy, Clone, PartialEq)]
pub enum HeroRarity {
  Unknown,
  N,
  R,
  SR,
  SSR,
  SP,
}

#[test]
fn test_parse() {
  use bridge_value::ParseClientValue;
  use serde_json::json;
  let v = json!([
    "5c50fcfb3938a3755e02c467",
    326,
    [
      "5abd73283938a30d12509516",
      "5a97785d9567c9ba582e0169",
      "5b7639e43938a3bae0b8b2dd",
      "5a98ae033938a338bb7f0da1",
      "5a99d68190d3fc7a0b2fb92b",
      "58d520d53938a3363e12cf92"
    ],
    40,
    0,
    1.0,
    "来摸一下",
    1548811515,
    true,
    5,
    [[3261, 5], [3262, 5], [3263, 5]]
  ]);
  let hero = Hero::parse_client_value(&v).unwrap();
  println!("{:#?}", hero)
}
