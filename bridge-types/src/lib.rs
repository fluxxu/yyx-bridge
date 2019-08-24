use bridge_derive::ParseClientValue;
use serde_derive::Serialize;

mod guild;
mod hero;
mod hero_book_shard;
mod hero_equip;
mod hero_equip_preset;
mod player;
mod realm_card;
mod story_task;

pub use self::guild::*;
pub use self::hero::*;
pub use self::hero_book_shard::*;
pub use self::hero_equip::*;
pub use self::hero_equip_preset::*;
pub use self::player::*;
pub use self::realm_card::*;
pub use self::story_task::*;

#[derive(Debug, Serialize, ParseClientValue)]
pub struct Snapshot {
  pub player: Player,
  pub currency: PlayerCurrency,
  pub heroes: Vec<Hero>,
  pub hero_equips: Vec<HeroEquip>,
  pub hero_equip_presets: Vec<HeroEquipPreset>,
  pub hero_book_shards: Vec<HeroBookShard>,
  pub realm_cards: Vec<RealmCard>,
  pub story_tasks: Vec<StoryTask>,
}
