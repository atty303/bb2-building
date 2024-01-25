extern crate rmp_serde;
extern crate serde;
extern crate strum;

pub use database::*;
pub use global::*;
pub use rune::*;
pub use search::*;
pub use skill::*;
pub use sprite::*;

mod database;
mod global;
mod rune;
mod search;
pub mod skill;
mod sprite;
pub mod state;
pub mod term;
pub mod token;

pub const LANGUAGES: [&str; 12] = [
    "ja", "en", "fr", "ko", "zh-CN", "zh-TW", "de", "es", "it", "ru", "pt", "pt-BR",
];
