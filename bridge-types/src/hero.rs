use bridge_derive::ParseClientValue;
use serde_derive::Serialize;

#[derive(Debug, Serialize, ParseClientValue)]
pub struct Hero {
  pub id: String,
  pub hero_id: i64,
  pub equips: Vec<String>,
  pub level: i64,
  pub exp: f64,
  pub nick_name: String,
  pub born: i64,
  pub lock: bool,
  pub rarity: HeroRarity,
  pub skills: Vec<HeroSkill>,
  pub awake: i64,
  pub star: i64,
  pub attrs: HeroAttrs,
}

#[derive(Debug, Serialize, ParseClientValue)]
pub struct HeroSkill {
  pub id: i64,
  pub level: i64,
}

#[derive(Debug, Serialize, ParseClientValue, Copy, Clone)]
pub enum HeroRarity {
  Unknown,
  N,
  R,
  SR,
  SSR,
  SP,
}

#[derive(Debug, Serialize, ParseClientValue)]
pub struct HeroAttrs {
  pub max_hp: HeroAttr,
  pub speed: HeroAttr,
  pub crit_power: HeroAttr,
  pub crit_rate: HeroAttr,
  pub defense: HeroAttr,
  pub attack: HeroAttr,
  pub effect_hit_rate: f64,
  pub effect_resist_rate: f64,
}

#[derive(Debug, Serialize, ParseClientValue)]
pub struct HeroAttr {
  pub base: f64,
  pub add_value: f64,
  pub add_rate: f64,
  pub value: f64,
}