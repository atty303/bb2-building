use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};

use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};

use ::{Database, Sprite};
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AvroSchema)]
pub enum AvoidType {
    None,
    C,
    A,
    LastHit,
}

impl AvoidType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "" => Some(AvoidType::None),
            "C" => Some(AvoidType::C),
            "A" => Some(AvoidType::A),
            "LastHit" => Some(AvoidType::LastHit),
            _ => None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AvroSchema)]
pub enum ParamKey {
    None,
    All,
    Random,
    RandomD,
    GearByPos,
    Act,
    Combat,
    LastAutoUse,
    Current,
    Master,
    Push,
    Debuffs,
    Buffs,
    Shadow,
    LastEnemy,
}

impl ParamKey {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "" => Some(ParamKey::None),
            "All" => Some(ParamKey::All),
            "Random" => Some(ParamKey::Random),
            "RandomD" => Some(ParamKey::RandomD),
            "GearByPos" => Some(ParamKey::GearByPos),
            "Act" => Some(ParamKey::Act),
            "Combat" => Some(ParamKey::Combat),
            "LastAutoUse" => Some(ParamKey::LastAutoUse),
            "Current" => Some(ParamKey::Current),
            "Master" => Some(ParamKey::Master),
            "Push" => Some(ParamKey::Push),
            "Debuffs" => Some(ParamKey::Debuffs),
            "Buffs" => Some(ParamKey::Buffs),
            "Shadow" => Some(ParamKey::Shadow),
            "LastEnemy" => Some(ParamKey::LastEnemy),
            _ => None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct Skill {
    pub hash: u16,
    pub id: String,
    pub modes: Vec<SkillMode>,
    // pub icon: String,
    pub category: SkillCategory,
    // pub poss_num: i8,
    // pub for_user: bool,
    // pub on_dict: bool,
    pub rarity: i8,
    // pub freq: i8,
    // pub aff1: i8,
    // pub aff2: i8,
    // pub aff3: i8,
    // pub aff4: i8,
    pub in_dictionary: bool,
    // pub drop: bool,
    // pub tag: String,
    pub is_free: bool,
}

impl Skill {
    pub fn name(&self) -> Tr { Tr::Name(self.modes[0].id.as_str()) }
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

impl SkillMode {
    pub fn name(&self) -> Tr { Tr::Name(self.id.as_str()) }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct Act {
    pub id: String,
    pub nodes: Vec<ActNode>,
}

impl Act {
    pub fn name(&self) -> Tr { Tr::Name(self.id.as_str()) }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct ActNode {
    pub id: String,
    pub action_type: String,
    pub target: i8,
    pub param_key: ParamKey,
    pub avoid_type: AvoidType,
    pub act_num: u8,
}

impl ActNode {
    pub fn name(&self) -> Tr { Tr::Name(self.id.as_str()) }
    pub fn action_type(&self) -> Tr { Tr::Action(self.action_type.as_str()) }

    pub fn format(&self, db: &Database) -> String {
        let last_hit = match self.avoid_type {
            AvoidType::LastHit => db.tr_str("DC-SkillNodeDesc-LastHit"),
            _ => "".to_string(),
        };
        let target = db.tr_str(format!("DC-SkillNodeDesc-TargetName-{}", self.target));
        let tg = match (self.target, &self.param_key) {
            (_, ParamKey::All) => db.tr_str("DC-SkillNodeDesc-TargetSkill-All"),
            (_, ParamKey::Random) => db.tr_str("DC-SkillNodeDesc-TargetSkill-Random"),
            (_, ParamKey::RandomD) => db.tr_str("DC-SkillNodeDesc-TargetSkill-RandomD"),
            (_, ParamKey::Current) => db.tr_str("DC-SkillNodeDesc-TargetSkill-Current"),
            (_, ParamKey::Buffs) => db.tr_str("DC-SkillNodeDesc-TargetSkill-Buffs"),
            (_, ParamKey::Debuffs) => db.tr_str("DC-SkillNodeDesc-TargetSkill-Debuffs"),
            (t, p) => format!("<tg {},{:?}>", t, p),
        };

        let template = db.tr(&Tr::Action(self.action_type.as_str()));
        let desc = template
            .replace("<lasthit>", last_hit.as_str())
            .replace("<t>", target.as_str())
            .replace("<tg>", tg.as_str())
            .replace("<dr>", db.tr(&Tr::Raw("WD-DamageType-Direct")).as_str());

        if self.act_num == 1 {
            desc
        } else {
            db.tr_str("DC-SkillNodeDesc-MultipleCase")
                .replace("{0}", desc.as_str())
                .replace("{1}", self.act_num.to_string().as_str())
        }
    }
}

#[derive(Clone, Default)]
pub struct SkillRepository {
    inner: HashMap<u16, Skill>,
    order: Vec<u16>,
}

impl SkillRepository {
    pub fn write<'a, W: Write, I: Iterator<Item = &'a Skill>>(avro_write: W, skills: I) -> Result<(), apache_avro::Error> {
        let schema = Skill::get_schema();
        let mut writer = apache_avro::Writer::with_codec(&schema, avro_write, apache_avro::Codec::Deflate);
        for skill in skills {
            writer.append_ser(skill)?;
        }
        Ok(())
    }

    pub fn read<R: Read>(avro_read: R) -> Result<Self, apache_avro::Error> {
        let reader = apache_avro::Reader::new(avro_read)?;
        let mut order = Vec::new();
        let mut map = HashMap::new();
        for result in reader {
            let value = &result.expect("Error reading value from avro reader");
            let r = apache_avro::from_value::<Skill>(&value).expect("Error deserializing value");
            let hash = r.hash;
            map.insert(hash, r);
            order.push(hash);
        }
        Ok(SkillRepository { inner: map, order })
    }

    pub fn iter(&self) -> impl Iterator<Item = &Skill> {
        self.order.iter().map(move |k| &self.inner[k])
    }
}

impl Deref for SkillRepository {
    type Target = HashMap<u16, Skill>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for SkillRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
