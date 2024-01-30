use data::Sprite;
use std::convert::TryInto;
use yaml_rust::{Yaml, YamlLoader};

pub fn parse_icon(name: &str) -> Sprite {
    let path = format!("dump/asset/ExportedProject/Assets/Sprite/{}.asset", name);
    let s = std::fs::read_to_string(path).unwrap();
    let docs = YamlLoader::load_from_str(s.as_str()).unwrap();
    let doc = &docs[0];
    let texture = &doc["Sprite"]["m_RD"]["texture"];
    let index = match texture["guid"].as_str().unwrap() {
        "62fd6711ce5ab1d42bbd93655ec0ab0c" => 1,
        "eb9d7fa2dbefb314ebcdfa2b4a55d666" => 2,
        "ebccdba04e8244a43927c6ee18b5eb33" => 3,
        id @ _ => panic!("invalid guid: {:?}", id),
    };
    // assert_eq!(
    //     texture["guid"].as_str().unwrap(),
    //     "ebccdba04e8244a43927c6ee18b5eb33"
    // );
    let texture_height = 4096;
    let rect = &doc["Sprite"]["m_Rect"];
    let x: Result<u16, _> = parse_number(&rect["x"]).try_into();
    let y: Result<u16, _> = parse_number(&rect["y"]).try_into();
    let width: Result<u8, _> = parse_number(&rect["width"]).try_into();
    let height: Result<u8, _> = parse_number(&rect["height"]).try_into();
    Sprite {
        index,
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
