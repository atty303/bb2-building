extern crate apache_avro;
extern crate serde;

use std::io::Read;
use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};
use term::Tr;

pub mod term;
pub mod skill;

pub const LANGUAGES: [&str; 12] =
    ["ja", "en", "fr", "ko", "zh-CN", "zh-TW", "de", "es", "it", "ru", "pt", "pt-BR"];

#[derive(Clone, Default)]
pub struct Database {
    term: term::TermMap,

    pub skill: skill::SkillRepository,
}

impl Database {
    pub fn read<R: Read>(skill_read: R) -> Result<Self, apache_avro::Error> {
        let skill = skill::SkillRepository::read(skill_read)?;
        Ok(Self {
            term: term::TermMap::new(),
            skill,
        })
    }

    pub fn set_term(&mut self, term: term::TermMap) {
        self.term = term;
    }

    pub fn tr(&self, key: &Tr) -> String {
        self.term.tr(key)
    }

    pub fn tr_str<S: AsRef<str>>(&self, key: S) -> String {
        self.term.tr(&Tr::Raw(key.as_ref()))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct Sprite {
    pub x: u16,
    pub y: u16,
    pub width: u8,
    pub height: u8,
}
