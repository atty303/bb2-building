extern crate apache_avro;
extern crate serde;
extern crate strum;

use std::io::{Read, Write};

use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};

pub mod term;
pub mod skill;
pub mod state;
pub mod token;

pub const LANGUAGES: [&str; 12] =
    ["ja", "en", "fr", "ko", "zh-CN", "zh-TW", "de", "es", "it", "ru", "pt", "pt-BR"];

#[derive(Clone, Default)]
pub struct Database {
    term: term::TermMap,

    pub skill: skill::SkillRepository,
    pub state: state::StateRepository,
}

impl Database {
    pub fn read<R: Read>(skill_read: R, state_read: R) -> Result<Self, apache_avro::Error> {
        let skill = skill::SkillRepository::read(skill_read)?;
        let state = state::StateRepository::read(state_read)?;
        Ok(Self {
            term: term::TermMap::new(),
            skill,
            state,
        })
    }

    pub fn term(&self) -> &term::TermMap {
        &self.term
    }

    pub fn set_term(&mut self, term: term::TermMap) {
        self.term = term;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct Sprite {
    pub x: u16,
    pub y: u16,
    pub width: u8,
    pub height: u8,
}

pub fn write_avro<'a, W: Write, T: AvroSchema + Serialize + 'a, I: Iterator<Item = &'a T>>(avro_write: W, items: I) -> Result<(), apache_avro::Error> {
    let schema = T::get_schema();
    let mut writer = apache_avro::Writer::with_codec(&schema, avro_write, apache_avro::Codec::Deflate);
    for item in items {
        writer.append_ser(item)?;
    }
    Ok(())
}

// pub fn read_avro<'a, R: Read, T: AvroSchema + Deserialize<'a> + 'a>(avro_read: R) -> Result<Vec<T>, apache_avro::Error> {
//     let mut out = Vec::new();
//     let schema = T::get_schema();
//     let reader = apache_avro::Reader::with_schema(&schema, avro_read)?;
//     for result in reader {
//         let value = result.expect("Error reading value from avro reader");
//         let v = value.to_owned();
//         let r = apache_avro::from_value::<T>(&v).expect("Error deserializing value");
//         out.push(r);
//     }
//     Ok(out)
// }
