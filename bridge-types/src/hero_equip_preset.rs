use bridge_derive::ParseClientValue;
use serde_derive::Serialize;

#[derive(Debug, Serialize, ParseClientValue)]
pub struct HeroEquipPreset {
  pub name: String,
  pub items: Vec<String>,
}
