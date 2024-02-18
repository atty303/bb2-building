use crate::search::SearchCatalogs;
use data::Database;
use dioxus::prelude::*;
use std::string::ToString;

pub static THEME: GlobalSignal<String> = Signal::global(|| "dark".to_string());
pub static DATABASE: GlobalSignal<Database> = Signal::global(|| Database::default());
pub static SEARCH_CATALOGS: GlobalSignal<SearchCatalogs> =
    Signal::global(|| SearchCatalogs::default());
