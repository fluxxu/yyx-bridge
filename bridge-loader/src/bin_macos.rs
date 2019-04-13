mod macos;
mod lib_macos;

fn main() {
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

  use std::ffi::CString;
  let self_dir = macos::get_self_dir();
  println!("Output dir: {:?}", self_dir);

  let cstr = CString::new(self_dir.to_string_lossy().as_bytes()).unwrap();
  lib_macos::run(cstr.as_ptr());
}