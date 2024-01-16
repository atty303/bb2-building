extern crate rmp_serde;
extern crate serde;
extern crate strum;

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
