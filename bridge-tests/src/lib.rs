#![feature(proc_macro_hygiene)]

#[test]
fn test_secret_string() {
  use bridge_derive::secret_string;
  assert_eq!(secret_string!("666"), "666");
}

#[test]
fn test_secret_string_from_file() {
  use bridge_derive::secret_string_from_file;
  assert_eq!(
    secret_string_from_file!("bridge-tests/src/test.txt"),
    "FROM FILE!"
  );
}
