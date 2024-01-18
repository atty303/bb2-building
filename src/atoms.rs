use fermi::Atom;

use crate::search::SearchRepository;
use data::Database;

pub static LANGUAGE: Atom<Option<String>> = Atom(|_| None);
pub static DATABASE: Atom<Database> = Atom(|_| Database::default());
pub static SEARCH_INDEX: Atom<SearchRepository> = Atom(|_| SearchRepository::default());
