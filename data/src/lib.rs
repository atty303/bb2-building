extern crate apache_avro;
extern crate serde;
extern crate strum;

use std::io::Read;

use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};

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
