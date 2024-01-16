use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GlobalRepository {
    pub rarity_colors: Vec<String>,
}

impl GlobalRepository {
    pub fn read<R: Read>(read: R) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::decode::from_read(read)
    }

    pub fn write<W: Write>(&self, write: &mut W) -> Result<(), rmp_serde::encode::Error> {
        rmp_serde::encode::write(write, self)
    }
}
