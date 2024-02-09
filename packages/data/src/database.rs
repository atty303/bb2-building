use std::io::{Read, Write};
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use {rune, term};
use {skill, GlobalRepository};

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Database {
    pub global: GlobalRepository,
    pub term: Rc<term::TermRepository>,
    pub skill: Rc<skill::SkillRepository>,
    pub rune: Rc<rune::RuneRepository>,
}

impl Database {
    pub fn read<R: Read>(read: R) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::decode::from_read(read)
    }

    pub fn write<W: Write>(&self, write: &mut W) -> Result<(), rmp_serde::encode::Error> {
        rmp_serde::encode::write(write, self)
    }
}
