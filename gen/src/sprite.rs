use std::convert::TryInto;
use yaml_rust::{Yaml, YamlLoader};
use data::Sprite;

pub fn parse_icon(name: &str) -> Sprite {
    let path = format!("dump/asset/ExportedProject/Assets/Sprite/{}.asset", name);
    let s = std::fs::read_to_string(path).unwrap();
    let docs = YamlLoader::load_from_str(s.as_str()).unwrap();
    let doc = &docs[0];
    let texture = &doc["Sprite"]["m_RD"]["texture"];
    assert_eq!(texture["guid"].as_str().unwrap(), "a50549b8827f09843841d13f031f165f");
    let texture_height = 4096;
    let rect = &doc["Sprite"]["m_Rect"];
    let x: Result<u16, _> = parse_number(&rect["x"]).try_into();
    let y: Result<u16, _> = parse_number(&rect["y"]).try_into();
    let width: Result<u8, _> = parse_number(&rect["width"]).try_into();
    let height: Result<u8, _> = parse_number(&rect["height"]).try_into();
    Sprite {
        x: x.unwrap(),
        y: texture_height - y.unwrap() - height.unwrap() as u16,
        width: width.unwrap(),
        height: height.unwrap(),
    }
}

fn parse_number(v: &Yaml) -> u64 {
    if let Some(i) = v.as_i64() {
        i as u64
    } else if let Some(f) = v.as_f64() {
        f.round() as u64
    } else {
        panic!("invalid number: {:?}", v);
    }
}
