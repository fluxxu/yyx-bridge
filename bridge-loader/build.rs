extern crate winres;

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("yyx.ico");
    res.set_manifest(include_str!("./manifest.xml"));
    res.compile().unwrap();
  }
}
