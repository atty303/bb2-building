use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};

use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};

use ::{Database, Sprite};
use term;
use term::{nodes_to_string, TermMap};

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
    pub fn name(&self, terms: &TermMap) -> String {
        terms.tr(format!("NM-{}", self.modes[0].id).as_str(), |nodes| nodes_to_string(nodes))
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

impl SkillMode {
    pub fn name(&self, terms: &TermMap) -> String {
        terms.tr(format!("NM-{}", self.id).as_str(), |nodes| nodes_to_string(nodes))
    }

    pub fn format(&self, db: &Database) -> Vec<Description> {
        let mut descs = vec![];

        let line1 = db.term().tr(
            if self.is_alt { "NM-SkillNodeDesc-ModeName-AltMode" } else { "NM-SkillNodeDesc-ModeName-Normal" },
            |n| n.format_cb(|s| match s {
                "0" =>
                    if self.is_brave {
                        db.term().tr("NM-SkillNodeDesc-ModeName-ForBrave", |n| n.format_none())
                    } else {
                        vec![Description::None]
                    }
                _ => vec![]
            }));
        descs.extend(line1);
        descs.push(Description::NewLine);

        for act in &self.acts {
            for node in &act.nodes {
                descs.extend(node.format(db));
                descs.push(Description::NewLine);
            }
        }

        descs
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct Act {
    pub id: String,
    pub nodes: Vec<ActNode>,
}

impl Act {
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
    pub fn format(&self, db: &Database) -> Vec<Description> {
        let mut descs = vec![];

        let line = db.term().tr(
            format!("DC-SkillNodeDesc-{}", self.action_type).as_str(),
            |n| n.format_cb(|s| match s {
                "lasthit" =>
                    match self.avoid_type {
                        AvoidType::LastHit => db.term().tr("DC-SkillNodeDesc-LastHit", |n| n.format_none()),
                        _ => vec![Description::None],
                    }
                "t" =>
                    db.term().tr(format!("DC-SkillNodeDesc-TargetName-{}", self.target).as_str(), |n| n.format_none()),
                "tg" =>
                    match self.param_key {
                        ParamKey::All => db.term().tr("DC-SkillNodeDesc-TargetSkill-All", |n| n.format_none()),
                        ParamKey::Random => db.term().tr("DC-SkillNodeDesc-TargetSkill-Random", |n| n.format_none()),
                        ParamKey::RandomD => db.term().tr("DC-SkillNodeDesc-TargetSkill-RandomD", |n| n.format_none()),
                        ParamKey::Current => db.term().tr("DC-SkillNodeDesc-TargetSkill-Current", |n| n.format_none()),
                        ParamKey::Buffs => db.term().tr("DC-SkillNodeDesc-TargetSkill-Buffs", |n| n.format_none()),
                        ParamKey::Debuffs => db.term().tr("DC-SkillNodeDesc-TargetSkill-Debuffs", |n| n.format_none()),
                        _ => vec![],
                    }
                "dr" =>
                    db.term().tr("WD-DamageType-Direct", |n| n.format_none()),
                _ => vec![],
            })
        );

        let line = if self.act_num == 1 {
            line
        } else {
            db.term().tr("DC-SkillNodeDesc-MultipleCase", |n| n.format_cb(|s| match s {
                "0" => line.clone(),
                "1" => vec![Description::Text(self.act_num.to_string())],
                _ => vec![],
            }))
        };

        descs.extend(line);
        descs
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

#[derive(Clone, PartialEq)]
pub enum Description {
    Text(String),
    MissingVar(String),
    NewLine,
    None,
}

type Nodes = Vec<term::Node>;
type Descs = Vec<Description>;

trait ToDescs {
    fn format_cb<F: Fn(&str) -> Vec<Description>>(&self, f: F) -> Vec<Description>;
    fn format_none(&self) -> Descs;
}

impl ToDescs for Nodes {
    fn format_cb<F: Fn(&str) -> Vec<Description>>(&self, f: F) -> Vec<Description> {
        self.iter().flat_map(|n| match n {
            term::Node::Text(s) => vec![Description::Text(s.clone())],
            term::Node::Var(s) => {
                let adds = f(s.as_str());
                if adds.is_empty() {
                    vec![Description::MissingVar(s.clone())]
                } else {
                    adds
                }
            }
            term::Node::NewLine => vec![Description::NewLine],
        }).collect::<Vec<_>>()
    }

    fn format_none(&self) -> Descs {
        self.iter().flat_map(|n| match n {
            term::Node::Text(s) => vec![Description::Text(s.clone())],
            term::Node::Var(s) => {
                vec![Description::Text(format!("${}", s))]
            }
            term::Node::NewLine => vec![Description::NewLine],
        }).collect::<Vec<_>>()
    }
}
