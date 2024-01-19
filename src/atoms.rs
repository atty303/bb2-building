use fermi::Atom;

use crate::search::SearchCatalogs;
use data::Database;

pub static LANGUAGE: Atom<Option<String>> = Atom(|_| None);
pub static DATABASE: Atom<Database> = Atom(|_| Database::default());
pub static SEARCH_CATALOGS: Atom<SearchCatalogs> = Atom(|_| SearchCatalogs::default());
