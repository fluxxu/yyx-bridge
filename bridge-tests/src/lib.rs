#![feature(proc_macro_hygiene)]

#[test]
fn it_works() {
  use bridge_derive::secret_string;
  assert_eq!(secret_string!("666"), "666");
}
