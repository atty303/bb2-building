use std::collections::HashMap;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};
use Sprite;
use term::Tr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AvroSchema)]
pub enum SkillCategory {
    Attack,
    Summon,
    Support,
    Survive,
    Special,
    Enemy,
}

impl SkillCategory {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0.Attack" => Some(SkillCategory::Attack),
            "1.Summon" => Some(SkillCategory::Summon),
            "2.Support" => Some(SkillCategory::Support),
            "3.Survive" => Some(SkillCategory::Survive),
            "4.Special" => Some(SkillCategory::Special),
            "9.Enemy" => Some(SkillCategory::Enemy),
            _ => None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct Skill {
    pub hash: u16,
    pub id: String,
    pub modes: Vec<SkillMode>,
    // pub name: String,
    // pub id: String,
    // pub icon: String,
    pub category: SkillCategory,
    // pub poss_num: i8,
    // pub for_user: bool,
    // pub on_dict: bool,
    // pub rarity: i8,
    // pub freq: i8,
    // pub aff1: i8,
    // pub aff2: i8,
    // pub aff3: i8,
    // pub aff4: i8,
    // pub in_dict: bool,
    // pub drop: bool,
    // pub tag: String,
    // pub is_free: bool,
}

impl Skill {
    pub fn name(&self) -> Tr { Tr::Name(self.modes[0].id.as_str()) }
}

#[derive(Clone, Default)]
pub struct SkillMap {
    inner: HashMap<u16, Skill>,
}

impl SkillMap {
    pub fn new() -> Self {
        let map = HashMap::new();
        Self { inner: map }
    }

    pub fn write<W: Write>(avro_write: W, skill_map: &Self) -> Result<(), apache_avro::Error> {
        let schema = Skill::get_schema();
        let mut writer = apache_avro::Writer::new(&schema, avro_write);
        for skill in skill_map.inner.values() {
            writer.append_ser(skill)?;
        }
        Ok(())
    }

    pub fn read<R: Read>(avro_read: R) -> Result<Self, apache_avro::Error> {
        let reader = apache_avro::Reader::new(avro_read)?;
        let mut map = HashMap::new();
        for result in reader {
            let value = &result.expect("Error reading value from avro reader");
            let r = apache_avro::from_value::<Skill>(&value).expect("Error deserializing value");
            map.insert(r.hash, r);
        }
        Ok(SkillMap { inner: map })
    }

}

impl Deref for SkillMap {
    type Target = HashMap<u16, Skill>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for SkillMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct SkillMode {
    pub id: String,
    pub icon: Sprite,
    pub is_alt: bool,
    pub is_brave: bool,
    pub use_num: i8,
    pub use_brave: i8,
    pub cooldown: i8,
    pub use_init: bool,
    pub is_quick: bool,
    pub acts: Vec<Act>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct Act {
    pub id: String,
    pub nodes: Vec<ActNode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct ActNode {
    pub id: String,
    pub action_type: String,
}