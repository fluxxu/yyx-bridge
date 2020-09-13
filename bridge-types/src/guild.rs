use bridge_derive::ParseClientValue;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ParseClientValue)]
pub struct Guild {
  pub id: String,
  pub creator_id: String,
  pub short_id: i64,
  pub active_rank: i64,
  pub pvp_rank: i64,
  pub active_members: i64,
  pub funds: f64,
  pub medals: i64,
  pub creation_time: i64,
  pub level: i64,
  pub pvp_score: i64,
  pub members: Vec<GuildMember>,
  pub server_id: i64,
}

#[derive(Debug, Serialize, Deserialize, ParseClientValue, Copy, Clone)]
pub enum GuildMemberPosition {
  Unknown,
  Leader,
  Officer,
  Member,
}

#[derive(Debug, Serialize, Deserialize, ParseClientValue)]
pub struct GuildMember {
  pub id: String,
  pub position: GuildMemberPosition,
  pub donate_times: i64,
  pub last_login_time: i64,
  pub join_time: i64,
  pub offline_time: i64,
  pub weekly_feats: i64,
  pub medals: i64,
  pub nickname: String,
  pub dg_times: i64,
  pub name: String,
  pub level: i64,
  pub receive_times: i64,
  pub total_feats: i64,
  pub pvp_score: i64,
  pub task_finished_day: i64,
  pub task_finished_week: i64,
}

#[test]
fn test_deserialize() {
  use bridge_value::ParseClientValue;
  use serde_json::{Value, from_slice};

  let value: Value = from_slice(include_bytes!("../test_data/guild.json") as &[u8]).unwrap();
  let g = Guild::parse_client_value(&value).unwrap();
  dbg!(&g);
}