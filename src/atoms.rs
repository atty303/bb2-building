use fermi::Atom;

use data::Database;

pub static LANGUAGE: Atom<Option<String>> = Atom(|_| None);
pub static DATABASE: Atom<Database> = Atom(|_| Database::default());
