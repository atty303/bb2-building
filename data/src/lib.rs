extern crate rmp_serde;
extern crate serde;
extern crate strum;

use std::io::Read;

pub use global::*;
pub use sprite::*;

mod global;
pub mod skill;
mod sprite;
pub mod state;
pub mod term;
pub mod token;

pub const LANGUAGES: [&str; 12] = [
    "ja", "en", "fr", "ko", "zh-CN", "zh-TW", "de", "es", "it", "ru", "pt", "pt-BR",
];

#[derive(Clone, Default)]
pub struct Database {
    pub global: GlobalRepository,
    pub skill: skill::SkillRepository,
}

impl Database {
    pub fn read<R: Read>(global_read: R, skill_read: R) -> Result<Self, rmp_serde::decode::Error> {
        let global = GlobalRepository::read(global_read)?;
        let skill = skill::SkillRepository::read(skill_read)?;
        Ok(Self { global, skill })
    }
}
