use data::Database;
use fermi::Atom;
use std::string::ToString;

pub static LANGUAGE: Atom<String> = Atom(|_| "".to_string());
pub static DATABASE: Atom<Database> = Atom(|_| Database::default());
