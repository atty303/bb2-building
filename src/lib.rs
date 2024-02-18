pub use app::App;
use derive_more::{Deref, DerefMut, Display, From, FromStr, Into};

mod app;
mod components;
mod editor;
mod global;
mod hooks;
mod pages;
mod search;
mod ui;
mod worker;

#[derive(Debug, Clone, PartialEq, Display, From, FromStr, Into, Deref, DerefMut)]
pub struct Language(String);
