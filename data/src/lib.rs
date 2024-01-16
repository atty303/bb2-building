extern crate rmp_serde;
extern crate serde;
extern crate strum;

use std::io::Read;

pub mod skill;
pub mod sprite;
pub mod state;
pub mod term;
pub mod token;

pub const LANGUAGES: [&str; 12] = [
    "ja", "en", "fr", "ko", "zh-CN", "zh-TW", "de", "es", "it", "ru", "pt", "pt-BR",
];

#[derive(Clone, Default)]
pub struct Database {
    term: term::TermRepository,

    pub skill: skill::SkillRepository,
}

impl Database {
    pub fn read<R: Read>(skill_read: R) -> Result<Self, rmp_serde::decode::Error> {
        let skill = skill::SkillRepository::read(skill_read)?;
        Ok(Self {
            term: term::TermRepository::default(),
            skill,
        })
    }

    pub fn term(&self) -> &term::TermRepository {
        &self.term
    }

    pub fn set_term(&mut self, term: term::TermRepository) {
        self.term = term;
    }
}
