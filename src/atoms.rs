use data::Database;
use fermi::Atom;

pub static DATABASE: Atom<Database> = Atom(|_| Database::default());
