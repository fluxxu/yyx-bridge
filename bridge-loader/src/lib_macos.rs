use bridge::PullResult;
use chrono::Local;
use std::os::raw::c_char;
use std::path::Path;
use std::io::{prelude::*, stdin};
use std::error::Error;

#[path = "./macos/mod.rs"]
mod macos;

pub fn run(out_dir: *const c_char) {
  use bridge::*;
  use std::ffi::CStr;

  let out_dir = unsafe { CStr::from_ptr(out_dir) }.to_string_lossy();
  let out_path = Path::new(&out_dir as &str);

  unsafe {
    let version = version_get();
    let version_str = CStr::from_ptr(version).to_string_lossy().to_string();
    version_free(version);

    println!("Bridge version: {}", version_str);
    println!("Generating snapshot...");

    let r = pull_emulator();
    if let Err(err) = handle_result(&out_path, &version_str, &r) {
      println!("Error: {}", err);

      wait_input();
    }
    pull_free(r);
  }
}

#[cfg(not(feature = "guild"))]
fn handle_result(out_path: &Path, version_str: &str, res: &PullResult) -> Result<(), Box<Error>> {
  use std::fs::write;

  if res.is_ok {
    use serde_json::{self, json};
    let now = Local::now();
    let ts = Local::now().format("%Y%m%d%H%M%S");

    let value: serde_json::Value = serde_json::from_str(&res.get_data_json().ok_or_else(|| "No data")?)?;
    let short_id: i64 = value
      .as_object()
      .and_then(|obj| {
        obj.get("player").and_then(|p| {
          p.as_object()
            .and_then(|p| p.get("id").and_then(|id| id.as_i64()))
        })
      })
      .unwrap_or(0);
    let path = format!("yyx_snapshot_{}_{}.json", ts, short_id);
    write(
      out_path.join(&path),
      serde_json::to_string_pretty(&json!({
        "timestamp": now,
        "version": version_str,
        "data": value
      }))?
    )?;
    println!("Snapshot generated: {}", &path);
    Ok(())
  } else {
    if let Some(data) = res.get_data_json() {
      write(out_path.join("last_error_data.json"), data).ok();
    }
    let msg = res
      .get_error_message()
      .unwrap_or_else(|| "Unknown error.".to_string());
    Err(msg.into())
  }
}

#[cfg(feature = "guild")]
fn handle_result(out_path: &Path, version_str: &str, res: &PullResult) -> Result<(), Box<Error>> {
  use bridge_types::{GuildMember, GuildMemberPosition};
  use csv::Writer;
  use serde_derive::Serialize;
  use std::fs::{write, File};

  #[derive(Serialize)]
  struct CsvRow {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "名称")]
    pub name: String,
    #[serde(rename = "昵称")]
    pub nickname: String,
    #[serde(rename = "等级")]
    pub level: i64,
    #[serde(rename = "职位")]
    pub position: String,
    #[serde(rename = "加入时间")]
    pub join_time: String,
    #[serde(rename = "上次登录时间")]
    pub last_login_time: String,
    #[serde(rename = "下线时间")]
    pub offline_time: String,
    #[serde(rename = "本周功勋")]
    pub weekly_feats: i64,
    #[serde(rename = "累计功勋")]
    pub total_feats: i64,
    #[serde(rename = "斗技积分")]
    pub pvp_score: i64,
    #[serde(rename = "累计勋章")]
    pub medals: i64,
    #[serde(rename = "赠送次数")]
    pub donate_times: i64,
    #[serde(rename = "受赠次数")]
    pub receive_times: i64,
    #[serde(rename = "今日任务完成次数")]
    pub task_finished_day: i64,
    #[serde(rename = "本周任务完成次数")]
    pub task_finished_week: i64,
    #[serde(rename = "dg_times")]
    pub dg_times: i64,
  }

  fn format_timestamp(t: i64) -> String {
    if t <= 0 {
      return "".to_string();
    }
    use chrono::TimeZone;
    Local
      .timestamp(t, 0)
      .format("%Y-%m-%d %H:%M:%S")
      .to_string()
  }

  if res.is_ok {
    use serde_json::{self, json};
    let now = Local::now();
    let ts = Local::now().format("%Y%m%d%H%M%S");

    let value: serde_json::Value =
      serde_json::from_str(&res.get_data_json().ok_or_else(|| "No data")?)?;
    let short_id: i64 = value
      .as_object()
      .and_then(|obj| obj.get("short_id").and_then(|id| id.as_i64()))
      .unwrap_or(0);
    let members: Vec<GuildMember> = serde_json::from_value(
      value
        .as_object()
        .and_then(|obj| obj.get("members"))
        .cloned()
        .unwrap_or(serde_json::Value::Null),
    )
    .unwrap_or(vec![]);
    let path = out_path.join(format!(
      "yyx_guild_snapshot_{}_{}_members.csv",
      ts, short_id
    ));
    let mut f = File::create(&path)?;
    // UTF8 BOM
    f.write_all(&[0xEF, 0xBB, 0xBF])?;
    let mut csv_writer = Writer::from_writer(f);
    for member in members {
      let row = CsvRow {
        id: member.id,
        position: match member.position {
          GuildMemberPosition::Leader => "会长",
          GuildMemberPosition::Officer => "副会长",
          GuildMemberPosition::Member => "成员",
          _ => "",
        }
        .to_string(),
        donate_times: member.donate_times,
        last_login_time: format_timestamp(member.last_login_time),
        join_time: format_timestamp(member.join_time),
        offline_time: format_timestamp(member.offline_time),
        weekly_feats: member.weekly_feats,
        medals: member.medals,
        nickname: member.nickname,
        dg_times: member.dg_times,
        name: member.name,
        level: member.level,
        receive_times: member.receive_times,
        total_feats: member.total_feats,
        pvp_score: member.pvp_score,
        task_finished_day: member.task_finished_day,
        task_finished_week: member.task_finished_week,
      };
      csv_writer.serialize(row)?;
    }
    drop(csv_writer);

    println!("Guild members csv generated: {:?}", &path);

    let path = out_path.join(format!("yyx_guild_snapshot_{}_{}.json", ts, short_id));

    write(
      &path,
      serde_json::to_string_pretty(&json!({
        "timestamp": now,
        "version": version_str,
        "data": value
      }))?,
    )?;

    println!("Guild snapshot json generated: {:?}", &path);    

    Ok(())
  } else {
    if let Some(data) = res.get_data_json() {
      write(out_path.join("last_error_data.json"), data).ok();
    }
    let msg = res
      .get_error_message()
      .unwrap_or_else(|| "Unknown error.".to_string());
    Err(msg.into())
  }
}

fn wait_input() {
  for _ in stdin().lock().bytes() {
    break;
  }
}
