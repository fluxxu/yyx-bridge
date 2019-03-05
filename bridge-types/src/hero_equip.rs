use bridge_derive::ParseClientValue;
use serde_derive::Serialize;

#[derive(Debug, Serialize, ParseClientValue, Clone, Copy, PartialEq)]
pub enum HeroEquipAttrType {
  Hp,
  Defense,
  Attack,
  HpRate,
  DefenseRate,
  AttackRate,
  Speed,
  CritRate,
  CritPower,
  EffectHitRate,
  EffectResistRate,
}

#[derive(Debug, Serialize, ParseClientValue, PartialEq)]
pub struct HeroEquip {
  pub id: String,
  pub suit_id: i64,
  pub quality: i64,
  pub pos: i64,
  pub equip_id: i64,
  pub level: i64,
  pub born: i64,
  pub lock: bool,
  pub garbage: bool,
  pub attrs: Vec<HeroEquipAttr>,
  pub base_attr: HeroEquipAttr,
  pub random_attrs: Vec<HeroEquipAttr>,
  pub random_attr_rates: Vec<HeroEquipAttr>,
  pub single_attrs: Vec<HeroEquipAttr>,
}

#[derive(Debug, Serialize, ParseClientValue, PartialEq)]
pub struct HeroEquipAttr {
  #[serde(rename = "type")]
  pub type_: HeroEquipAttrType,
  pub value: f64,
}

#[test]
fn test_parse() {
  use bridge_value::ParseClientValue;
  use serde_json::json;
  let v = json!([
    "5bf9f3c73938a3804eb86164",
    300050,
    6,
    5,
    160006,
    0,
    1543125527,
    false,
    false,
    [
      [2, 24.321088272004143],
      [3, 0.024148625948199814],
      [4, 0.028486227490220478]
    ],
    [5, 0.1],
    [
      [2, 24.321088272004143],
      [3, 0.024148625948199814],
      [4, 0.028486227490220478]
    ],
    [
      [4, 0.9495409163406826],
      [3, 0.8049541982733271],
      [2, 0.9007810471112646]
    ]
  ]);
  let equip = HeroEquip::parse_client_value(&v).unwrap();
  assert_eq!(
    equip,
    HeroEquip {
      id: "5bf9f3c73938a3804eb86164".to_owned(),
      suit_id: 300050,
      quality: 6,
      pos: 5,
      equip_id: 160006,
      level: 0,
      born: 1543125527,
      lock: false,
      garbage: false,
      attrs: vec![
        HeroEquipAttr {
          type_: HeroEquipAttrType::Attack,
          value: 24.321088272004143
        },
        HeroEquipAttr {
          type_: HeroEquipAttrType::HpRate,
          value: 0.024148625948199814
        },
        HeroEquipAttr {
          type_: HeroEquipAttrType::DefenseRate,
          value: 0.028486227490220478
        }
      ],
      base_attr: HeroEquipAttr {
        type_: HeroEquipAttrType::AttackRate,
        value: 0.1
      },
      random_attrs: vec![
        HeroEquipAttr {
          type_: HeroEquipAttrType::Attack,
          value: 24.321088272004143
        },
        HeroEquipAttr {
          type_: HeroEquipAttrType::HpRate,
          value: 0.024148625948199814
        },
        HeroEquipAttr {
          type_: HeroEquipAttrType::DefenseRate,
          value: 0.028486227490220478
        }
      ],
      random_attr_rates: vec![
        HeroEquipAttr {
          type_: HeroEquipAttrType::DefenseRate,
          value: 0.9495409163406826
        },
        HeroEquipAttr {
          type_: HeroEquipAttrType::HpRate,
          value: 0.8049541982733271
        },
        HeroEquipAttr {
          type_: HeroEquipAttrType::Attack,
          value: 0.9007810471112646
        }
      ]
    }
  )
}
