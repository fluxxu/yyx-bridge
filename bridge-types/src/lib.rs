use bridge_derive::ParseClientValue;
use serde_derive::Serialize;

mod hero;
mod hero_book_fragment;
mod hero_equip;
mod hero_equip_preset;

pub use self::hero::*;
pub use self::hero_book_fragment::*;
pub use self::hero_equip::*;
pub use self::hero_equip_preset::*;

#[derive(Debug, Serialize, ParseClientValue)]
pub struct Snapshot {
  pub heroes: Vec<Hero>,
  pub hero_equips: Vec<HeroEquip>,
  pub hero_equip_presets: Vec<HeroEquipPreset>,
  pub hero_book_fragments: Vec<HeroBookFragment>,
}
