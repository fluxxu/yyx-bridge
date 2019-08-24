use chrono::Local;
use libloading::{self, Library, Symbol};
use std::error::Error;
use std::io::prelude::*;
use std::io::stdin;
use std::os::raw::c_char;
use std::ptr;

struct LibInterface<'a> {
  pull_windows: Symbol<'a, extern "C" fn() -> PullResult>,
  pull_emulator: Symbol<'a, extern "C" fn() -> PullResult>,
  pull_free: Symbol<'a, extern "C" fn(PullResult)>,
  version_get: Symbol<'a, extern "C" fn() -> *mut c_char>,
  version_free: Symbol<'a, extern "C" fn(*mut c_char)>,
}

#[derive(Debug)]
pub struct PullResult {
  pub is_ok: bool,
  pub error_message: *mut c_char,
  pub data_json: *mut c_char,
}

impl PullResult {
  fn get_error_message(&self) -> Option<String> {
    use std::ffi::CString;
    if self.error_message != ptr::null_mut() {
      let cstr = unsafe { CString::from_raw(self.error_message) };
      let v = cstr.to_string_lossy().to_string();
      cstr.into_raw();
      Some(v)
    } else {
      None
    }
  }

  fn get_data_json(&self) -> Option<String> {
    use std::ffi::CString;
    if self.data_json != ptr::null_mut() {
      let cstr = unsafe { CString::from_raw(self.data_json) };
      let v = cstr.to_string_lossy().to_string();
      cstr.into_raw();
      Some(v)
    } else {
      None
    }
  }
}

fn main() {
  use std::env;
  use std::ffi::CStr;

  #[cfg(not(feature = "mumu"))]
  let is_emu_mode = env::args().any(|item| item == "-emu");
  #[cfg(feature = "mumu")]
  let is_emu_mode = true;

  #[cfg(not(feature = "guild"))]
  {
    println!("痒痒熊快照");
    println!("https://nga.178.com/read.php?tid=16557282");
  }

  #[cfg(feature = "guild")]
  {
    println!("痒痒熊寮快照");
    println!("https://nga.178.com/read.php?tid=16941381");
    println!(
      "注意: 要导出寮任务完成情况，你需要在游戏内打开 阴阳寮 -> 神社 -> 集体任务 -> 任务完成记录"
    );
    println!("确保屏幕上的数字加载完成后再运行本程序")
  }

  println!("****************************************");

  println!("装载中...");

  #[cfg(not(feature = "guild"))]
  let dllname = { "bridge.dll" };

  #[cfg(feature = "guild")]
  let dllname = { "bridge-guild.dll" };

  let lib = match libloading::Library::new(dllname) {
    Ok(lib) => lib,
    Err(err) => {
      println!("装载bridge失败: {}", err);
      wait_input();
      return;
    }
  };
  let LibInterface {
    pull_windows,
    pull_emulator,
    pull_free,
    version_get,
    version_free,
  } = get_symbols(&lib);
  let version = version_get();
  let version_str = unsafe { CStr::from_ptr(version).to_string_lossy().to_string() };
  version_free(version);

  println!("Bridge版本: {}", version_str);
  println!("正在生成快照...");

  let r = if is_emu_mode {
    pull_emulator()
  } else {
    pull_windows()
  };
  if let Err(err) = handle_result(&version_str, &r) {
    println!("错误: {}", err);
    wait_input();
  } else {
    println!("快照已生成。");
    ::std::thread::sleep(::std::time::Duration::from_secs(3));
  }
  pull_free(r);
}

fn get_symbols<'a>(lib: &'a Library) -> LibInterface<'a> {
  unsafe {
    LibInterface {
      pull_windows: lib.get(b"pull_windows").unwrap(),
      pull_emulator: lib.get(b"pull_emulator").unwrap(),
      pull_free: lib.get(b"pull_free").unwrap(),
      version_get: lib.get(b"version_get").unwrap(),
      version_free: lib.get(b"version_free").unwrap(),
    }
  }
}

#[cfg(not(feature = "guild"))]
fn handle_result(version_str: &str, res: &PullResult) -> Result<(), Box<Error>> {
  use std::fs::write;

  if res.is_ok {
    use serde_json::{self, json};
    let now = Local::now();
    let ts = Local::now().format("%Y%m%d%H%M%S");

    let value: serde_json::Value =
      serde_json::from_str(&res.get_data_json().ok_or_else(|| "No data")?)?;
    let (short_id, server_id): (i64, i64) = value
      .as_object()
      .and_then(|obj| {
        obj.get("player").and_then(|p| {
          p.as_object()
            .and_then(|p| Some((p.get("id")?.as_i64()?, p.get("server_id")?.as_i64()?)))
        })
      })
      .unwrap_or((0, 0));
    write(
      format!("yyx_snapshot_{}_{}_{}.json", ts, server_id, short_id),
      serde_json::to_string_pretty(&json!({
        "timestamp": now,
        "version": version_str,
        "data": value
      }))?,
    )?;
    Ok(())
  } else {
    if let Some(data) = res.get_data_json() {
      write("last_error_data.json", data).ok();
    }
    let msg = res
      .get_error_message()
      .unwrap_or_else(|| "Unknown error.".to_string());
    Err(msg.into())
  }
}

#[cfg(feature = "guild")]
fn handle_result(version_str: &str, res: &PullResult) -> Result<(), Box<Error>> {
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
    #[serde(rename = "道馆完成次数")]
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
    let (short_id, server_id): (i64, i64) = value
      .as_object()
      .and_then(|obj| {
        Some((
          obj.get("short_id")?.as_i64()?,
          obj.get("server_id")?.as_i64()?,
        ))
      })
      .unwrap_or((0, 0));
    let members: Vec<GuildMember> = serde_json::from_value(
      value
        .as_object()
        .and_then(|obj| obj.get("members"))
        .cloned()
        .unwrap_or(serde_json::Value::Null),
    )
    .unwrap_or(vec![]);
    let mut f = File::create(format!(
      "yyx_guild_snapshot_{}_{}_{}_members.csv",
      ts, server_id, short_id
    ))?;
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
    write(
      format!("yyx_guild_snapshot_{}_{}_{}.json", ts, server_id, short_id),
      serde_json::to_string_pretty(&json!({
        "timestamp": now,
        "version": version_str,
        "data": value
      }))?,
    )?;
    Ok(())
  } else {
    if let Some(data) = res.get_data_json() {
      write("last_error_data.json", data).ok();
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
