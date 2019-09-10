use bridge_derive::ParseClientValue;
use serde_derive::Serialize;

#[derive(Debug, Serialize, ParseClientValue, PartialEq)]
pub struct Player {
  pub id: i64,
  pub server_id: i64,
  pub name: String,
  pub level: i64,
}

#[derive(Debug, Serialize, ParseClientValue, PartialEq)]
pub struct PlayerCurrency {
  pub coin: i64,
  pub jade: i64,
  pub action_point: i64,
  pub auto_point: i64,
  pub honor: i64,
  pub medal: i64,
  pub contrib: i64,
  pub totem_pass: i64,
  pub s_jade: i64,
  pub skin_token: i64,
  pub realm_raid_pass: i64,
  pub broken_amulet: i64,
  pub mystery_amulet: i64,
  pub ar_amulet: i64,
  pub ofuda: i64,
  pub gold_ofuda: i64,
  pub scale: i64,
  pub reverse_scale: i64,
  pub demon_soul: i64,
  pub foolery_pass: i64,
  pub sp_skin_token: i64,
}
