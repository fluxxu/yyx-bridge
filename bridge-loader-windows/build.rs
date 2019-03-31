extern crate winres;

fn main() {
  let mut res = winres::WindowsResource::new();
  res.set_icon("./yyx.ico");
  #[cfg(not(feature = "steam"))]
  res.set_manifest(include_str!("./manifest.xml"));
  res.compile().unwrap();
}
