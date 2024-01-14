use fermi::Atom;
use data::Database;

pub static DATABASE: Atom<Database> = Atom(|_| Database::default());
