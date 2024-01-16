use data::Database;
use fermi::Atom;
use std::string::ToString;

pub static LANGUAGE: Atom<Option<String>> = Atom(|_| None);
pub static DATABASE: Atom<Database> = Atom(|_| Database::default());
