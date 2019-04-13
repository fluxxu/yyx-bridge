extern crate winres;

fn main() {
  let mut res = winres::WindowsResource::new();
  #[cfg(not(feature = "guild"))]
  res.set_icon("./yyx.ico");
  #[cfg(feature = "guild")]
  res.set_icon("./yyx-guild.ico");
  #[cfg(not(feature = "noadmin"))]
  res.set_manifest(include_str!("./manifest.xml"));
  res.compile().unwrap();
}
