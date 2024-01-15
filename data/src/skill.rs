use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};

use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use ::{Database, Sprite};
use term;
use term::{Node, nodes_to_string, TermMap};

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
    A,
    C,
    LastHit,
}

impl AvoidType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "" => Some(AvoidType::None),
            "A" => Some(AvoidType::A),
            "C" => Some(AvoidType::C),
            "LastHit" => Some(AvoidType::LastHit),
            _ => None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AvroSchema, EnumString, Display)]
pub enum ParamKey {
    #[strum(serialize = "")]
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

    pub fn format(&self, db: &Database) -> Vec<Node> {
        let mut nodes = vec![];

        let line1 = db.term().tr(
            if self.is_alt { "NM-SkillNodeDesc-ModeName-AltMode" } else { "NM-SkillNodeDesc-ModeName-Normal" },
            |n| n.map_var(|s| match s {
                "0" =>
                    if self.is_brave {
                        db.term().get("NM-SkillNodeDesc-ModeName-ForBrave")
                    } else {
                        vec![Node::Empty]
                    }
                _ => vec![]
            }));
        nodes.extend(line1);
        nodes.push(Node::NewLine);

        for act in &self.acts {
            nodes.extend(act.format(db));
            nodes.push(Node::NewLine);
        }

        nodes
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema, EnumString, Display)]
pub enum ActTrigger {
    OnUse,
    TurnStart,
    Passive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct Act {
    pub id: String,
    pub act_trigger: ActTrigger,
    pub nodes: Vec<ActNode>,
}

impl Act {
    pub fn format(&self, db: &Database) -> Vec<Node> {
        let mut nodes = vec![];

        nodes.extend(db.term.get(format!("NM-SkillNodeDesc-ActTrigger-{}", self.act_trigger).as_str()));
        nodes.push(Node::NewLine);

        for node in &self.nodes {
            nodes.extend(node.format(db));
            nodes.push(Node::NewLine);
        }
        nodes
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct ActNode {
    pub id: String,
    pub action_type: String,
    pub target: i8,
    pub param_key: ParamKey,
    pub hit_rate: u16,
    pub avoid_type: AvoidType,
    pub act_num: u8,
    pub crit_rate: u16,
}

impl ActNode {
    fn replacer(&self, name: &str, db: &Database, out: &mut Vec<Node>) {
        match name {
            "lasthit" =>
                match self.avoid_type {
                    AvoidType::LastHit => out.extend(db.term().get("DC-SkillNodeDesc-LastHit")),
                    _ => out.push(Node::Empty)
                }
            "t" =>
                out.extend(db.term().get(format!("DC-SkillNodeDesc-TargetName-{}", self.target).as_str())),
            "tg" =>
                out.extend(db.term().get(format!("DC-SkillNodeDesc-TargetSkill-{}", self.param_key).as_str())),
            "dr" =>
                out.extend(db.term().get("WD-DamageType-Direct")),
            "accu" => {
                match self.avoid_type {
                    AvoidType::None => out.push(Node::Error("$accu->None".to_string())),
                    AvoidType::A => {
                        out.push(Node::NewLine);
                        out.push(Node::Text("　".to_string()));
                        out.extend(db.term().get("DC-SkillNodeDesc-AvoidType-A"));
                    }
                    AvoidType::C => {
                        out.push(Node::NewLine);
                        out.push(Node::Text("　".to_string()));
                        out.extend(db.term().get("DC-SkillNodeDesc-AvoidType-C"));
                    }
                    AvoidType::LastHit => out.push(Node::Error("$accu->LastHit".to_string())),
                }
            }
            "hit" =>
                out.push(Node::Text(self.hit_rate.to_string())),
            "crit" =>
                if self.crit_rate == 0 || self.crit_rate == 100 {
                    out.push(Node::Empty)
                } else {
                    out.push(Node::NewLine);
                    out.push(Node::Text("　".to_string()));
                    out.extend(db.term().tr("DC-SkillNodeDesc-CritRate", |n| n.map_var(|s| match s {
                        "0" => vec![Node::Text(self.crit_rate.to_string())],
                        _ => vec![],
                    })));
                }
            "power" =>
                out.extend(db.term().get("DC-SkillNodeDesc-AboutPower")),
            _ => ()
        }
    }

    pub fn format(&self, db: &Database) -> Vec<Node> {
        let replacer = |s: &str| {
            let mut out = vec![];
            self.replacer(s, db, &mut out);
            out
        };

        let mut line: Vec<Node> = db.term().get(format!("DC-SkillNodeDesc-{}", self.action_type).as_str());
        loop {
            let has_var = line.iter().any(|l| match l {
                Node::Var(_) => true,
                _ => false,
            });
            if !has_var {
                break;
            }

            let mut replaced = false;
            line = line.into_iter().flat_map(|l| match l {
                Node::Var(s) => {
                    let n = replacer(s.as_str());
                    if n.is_empty() {
                        vec![Node::Var(s.clone())]
                    } else {
                        replaced = true;
                        n
                    }
                }
                _ => vec![l.clone()],
            }).collect::<Vec<_>>();
            if !replaced {
                break;
            }
        }

        let line = if self.act_num == 1 {
            line
        } else {
            db.term().tr("DC-SkillNodeDesc-MultipleCase", |n| n.map_var(|s| match s {
                "0" => line.clone(),
                "1" => vec![Node::Text(self.act_num.to_string())],
                _ => vec![],
            }))
        };

        line
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

type Nodes = Vec<term::Node>;

trait ToDescs {
    fn map_var<F: Fn(&str) -> Vec<Node>>(&self, f: F) -> Vec<Node>;
}

impl ToDescs for Nodes {
    fn map_var<F: Fn(&str) -> Vec<Node>>(&self, f: F) -> Vec<Node> {
        self.iter().flat_map(|n| match n {
            term::Node::Var(s) => {
                let adds = f(s.as_str());
                if adds.is_empty() {
                    vec![n.clone()]
                } else {
                    adds
                }
            }
            n => vec![n.clone()],
        }).collect::<Vec<_>>()
    }
}
