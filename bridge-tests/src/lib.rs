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

#[test]
fn test_derive_from_client_value_enum() {
  use bridge_derive::ParseClientValue;
  use bridge_value::{ParseClientValue, Value};

  #[derive(ParseClientValue, Debug, PartialEq, Clone, Copy)]
  enum T {
    V0,
    V1,
    V2,
    V3,
  }

  let v = Value::Array(vec![0, 1, 2, 3].into_iter().map(Into::into).collect());
  let vv = Vec::<T>::parse_client_value(&v).unwrap();
  assert_eq!(vv, vec![T::V0, T::V1, T::V2, T::V3]);
}

#[test]
fn test_derive_from_client_value_struct() {
  use bridge_derive::ParseClientValue;
  use bridge_value::ParseClientValue;
  use serde_json::json;

  #[derive(ParseClientValue, Debug, PartialEq)]
  struct T {
    v0: i64,
    v1: i64,
    v2: i64,
    v3: i64,
    tt: TT,
  }

  #[derive(ParseClientValue, Debug, PartialEq)]
  struct TT {
    v: String,
  }

  let v = json!([0, 1, 2, 3, ["WTF"]]);
  let vv = T::parse_client_value(&v).unwrap();
  assert_eq!(
    vv,
    T {
      v0: 0,
      v1: 1,
      v2: 2,
      v3: 3,
      tt: TT {
        v: "WTF".to_string()
      }
    }
  );
}
