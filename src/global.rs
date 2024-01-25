use crate::search::SearchCatalogs;
use data::Database;
use dioxus::prelude::*;

pub static LANGUAGE: GlobalSignal<Option<String>> = Signal::global(|| None);
pub static DATABASE: GlobalSignal<Database> = Signal::global(|| Database::default());
pub static SEARCH_CATALOGS: GlobalSignal<SearchCatalogs> =
    Signal::global(|| SearchCatalogs::default());
