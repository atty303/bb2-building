use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut, Range};

use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use ::{Database, Sprite};
use term::{nodes_to_string, TermMap};
use token::Token;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AvroSchema, EnumString, Display)]
pub enum Target {
    SELF,
    TARGET,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AvroSchema, EnumString, Display)]
pub enum Reduce {
    #[strum(serialize = "")]
    None,
    P,
    M,
    V,
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

    pub poss_num: i8, // copy from Skill
}

impl SkillMode {
    pub fn name(&self, terms: &TermMap) -> String {
        terms.tr(format!("NM-{}", self.id).as_str(), |nodes| nodes_to_string(nodes))
    }

    pub fn format(&self, db: &Database) -> Vec<Token> {
        let mut nodes = vec![];

        let line1 = db.term().tr(
            if self.is_alt { "NM-SkillNodeDesc-ModeName-AltMode" } else { "NM-SkillNodeDesc-ModeName-Normal" },
            |n| n.map_var(|s| match s {
                "0" =>
                    if self.is_brave {
                        db.term().get("NM-SkillNodeDesc-ModeName-ForBrave")
                    } else {
                        vec![Token::Empty]
                    }
                _ => vec![]
            }));
        nodes.extend(line1);
        nodes.push(Token::NewLine);

        for act in &self.acts {
            nodes.extend(act.format(db));
            nodes.push(Token::NewLine);
        }

        nodes.extend(db.term().get("WD-Cooldown"));
        nodes.push(Token::Text(format!(": {}", self.cooldown)));
        nodes.push(Token::NewLine);

        nodes.extend(db.term().get("WD-SkillPossRemain"));
        nodes.push(Token::Text(format!(": -{}/{}", self.use_num, self.poss_num)));
        nodes.push(Token::NewLine);

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
    pub fn format(&self, db: &Database) -> Vec<Token> {
        let mut nodes = vec![];

        nodes.extend(db.term.get(format!("NM-SkillNodeDesc-ActTrigger-{}", self.act_trigger).as_str()));
        nodes.push(Token::NewLine);

        for node in &self.nodes {
            nodes.extend(node.format(db));
            nodes.push(Token::NewLine);
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
    pub state_row_id: Option<String>,
    pub hit_rate: u16,
    pub avoid_type: AvoidType,
    pub relate_target: Target,
    pub relate: String,
    pub power: u32,
    pub reduce: Reduce,
    pub inc_target: Target,
    pub inc_relate: String,
    pub inc_power: u16,
    pub state_last: StateLast,
    pub act_num: u8,
    pub crit_rate: u16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct StateLast {
    pub f1: i8,
    pub f2: i8, // TODO: turn
    pub f3: i8,
    pub room: i8,
    pub f5: i8,
}

impl ActNode {
    fn replacer(&self, name: &str, db: &Database, out: &mut Vec<Token>) {
        match name {
            "lasthit" =>
                match self.avoid_type {
                    AvoidType::LastHit => out.extend(db.term().get("DC-SkillNodeDesc-LastHit")),
                    _ => out.push(Token::Empty)
                }
            "t" => {
                let target = if self.target < 0 { 0 } else { self.target };
                out.extend(db.term().get(format!("DC-SkillNodeDesc-TargetName-{}", target).as_str()));
            }
            "tg" =>
                // TODO: param_key じゃないっぽい(-1)
                out.extend(db.term().get(format!("DC-SkillNodeDesc-TargetSkill-{}", self.param_key).as_str())),
            "dr" =>
                out.extend(db.term().get("WD-DamageType-Direct")),
            "accu" => {
                match self.avoid_type {
                    AvoidType::None => {
                        out.push(Token::NewLine);
                        out.push(Token::Text("　".to_string()));
                        out.extend(db.term().get("DC-SkillNodeDesc-AvoidType-"));
                    }
                    AvoidType::A => {
                        out.push(Token::NewLine);
                        out.push(Token::Text("　".to_string()));
                        out.extend(db.term().get("DC-SkillNodeDesc-AvoidType-A"));
                    }
                    AvoidType::C => {
                        out.push(Token::NewLine);
                        out.push(Token::Text("　".to_string()));
                        out.extend(db.term().get("DC-SkillNodeDesc-AvoidType-C"));
                    }
                    AvoidType::LastHit => {
                        out.push(Token::NewLine);
                        out.push(Token::Text("　".to_string()));
                        out.extend(db.term().get("DC-SkillNodeDesc-AvoidType-"));
                        // out.push(Node::Empty),
                    }
                }
            }
            "hit" =>
                out.push(Token::Text(self.hit_rate.to_string())),
            "crit" =>
                if self.crit_rate == 0 || self.crit_rate == 100 {
                    out.push(Token::Empty)
                } else {
                    out.push(Token::NewLine);
                    out.push(Token::Text("　".to_string()));
                    out.extend(db.term().tr("DC-SkillNodeDesc-CritRate", |n| n.map_var(|s| match s {
                        "0" => vec![Token::Text(self.crit_rate.to_string())],
                        _ => vec![],
                    })));
                }
            "rd" =>
                match self.reduce {
                    Reduce::None => out.push(Token::Empty),
                    Reduce::P | Reduce::M | Reduce::V => {
                        out.push(Token::NewLine);
                        out.push(Token::Text("　".to_string()));
                        out.extend(db.term().get(format!("DC-SkillNodeDesc-Reduce-{}", self.reduce).as_str()));
                    }
                }
            "inc" => {
                if self.inc_relate.is_empty() {
                    out.push(Token::Empty)
                } else {
                    out.push(Token::NewLine);
                    out.push(Token::Text("　".to_string()));
                    let pair = self.inc_relate.split(':').collect::<Vec<_>>();
                    let key = pair[0];
                    match key {
                        "CritRate" =>
                            out.extend(db.term().get("DC-SkillNodeDesc-AboutIncPower")),
                        _ => (),
                    }
                }
            }
            "irt" =>
                match self.inc_target {
                    Target::SELF => out.extend(db.term().get("DC-SkillNodeDesc-TargetName-0")),
                    Target::TARGET => out.extend(db.term().get("DC-SkillNodeDesc-TargetName-1")),
                }
            "irf" => {
                let pair = self.inc_relate.split(':').collect::<Vec<_>>();
                let key = pair[0];
                out.extend(db.term().get(format!("NM-{}", key).as_str()));
            }
            "ipw" =>
                out.push(Token::Text(format!("{}", self.inc_power))),
            "power" =>
                out.extend(db.term().get("DC-SkillNodeDesc-AboutPower")),
            "pwd" => // rt,rf,pw
                out.extend(db.term().get("DC-SkillNodeDesc-AboutPowerDtl")),
            "rt" =>
                match self.inc_target {
                    Target::SELF => out.extend(db.term().get("DC-SkillNodeDesc-TargetName-0")),
                    Target::TARGET => out.extend(db.term().get("DC-SkillNodeDesc-TargetName-1")),
                }
            "rf" => {
                if self.relate.contains('/') {
                    let mut it = self.relate.split('/');
                    let or = [it.next().unwrap(), it.next().unwrap()].iter().map(|s| {
                        let n = &s[3..4];
                        db.term().get(format!("NM-MainParam:{}", n).as_str())
                    }).collect::<Vec<_>>();
                    out.extend(or[0].to_owned());
                    out.extend(db.term().get("WD-Relate-Or"));
                    out.extend(or[1].to_owned());
                } else {
                    let pair = self.relate.split(':').collect::<Vec<_>>();
                    let key = pair[0];
                    out.extend(db.term().get(format!("NM-{}", key).as_str()));
                }
            }
            "pw" =>
                out.push(Token::Text(format!("{}", self.power))),
            "last" => {
                if self.state_last.room >= 0 {
                    out.push(Token::NewLine);
                    out.push(Token::Text("　".to_string()));
                    out.extend(db.term().tr("DC-SkillNodeDesc-LastCombine", |n| n.map_var(|s| match s {
                        "0" => {
                            db.term().tr("DC-SkillNodeDesc-LastRoom", |n| n.map_var(|s| match s {
                                "0" => {
                                    vec![Token::Text(self.state_last.room.to_string())]
                                }
                                _ => vec![],
                            }))
                        },
                        _ => vec![],
                    })));
                } else if self.state_last.f1 >= 0 {
                    out.push(Token::Error("state_last.f1".to_string()));
                } else if self.state_last.f2 >= 0 {
                    out.push(Token::NewLine);
                    out.push(Token::Text("　".to_string()));
                    out.extend(db.term().tr("DC-SkillNodeDesc-LastCombine", |n| n.map_var(|s| match s {
                        "0" => {
                            db.term().tr("DC-SkillNodeDesc-LastTurn", |n| n.map_var(|s| match s {
                                "0" => {
                                    vec![Token::Text(self.state_last.f2.to_string())]
                                }
                                _ => vec![],
                            }))
                        },
                        _ => vec![],
                    })));
                } else if self.state_last.f3 >= 0 {
                    out.push(Token::Error("state_last.f3".to_string()));
                } else if self.state_last.f5 >= 0 {
                    out.push(Token::Error("state_last.f5".to_string()));
                } else {
                    out.push(Token::Empty);
                }
            }
            "st" => {
                if let Some(state_row_id) = &self.state_row_id {
                    if let Some(state) = db.state.get(state_row_id) {
                        out.extend(db.term().get(&format!("NM-{}", &state.id)))
                    } else {
                        out.push(Token::Error(format!("state not found: {}", state_row_id)));
                    }
                }
            }
            "srpw" =>
                if let Some(state_row_id) = &self.state_row_id {
                    if let Some(state) = db.state.get(state_row_id) {
                        let text = state.format.replace("{v}", &format!("{}", self.power));
                        out.push(Token::Text(text));
                    } else {
                        out.push(Token::Error(format!("state not found: {}", state_row_id)));
                    }
                }
            "stpw" =>
                out.push(Token::Empty), // TODO:
            "md" =>
                if self.action_type == "AltMode" {
                    if self.power == 0 {
                        out.extend(db.term().get("WD-SkillAltModeName-0"));
                    } else if self.power == 1 {
                        out.extend(db.term().get("WD-SkillAltModeName-1"));
                    } else {
                        out.push(Token::Error(format!("invalid power {}", self.power)));
                    }
                } else {
                    out.push(Token::Error(format!("invalid action_type {}", self.action_type)));
                }

            _ => (),
        }
    }

    pub fn format(&self, db: &Database) -> Vec<Token> {
        let replacer = |s: &str| {
            let mut out = vec![];
            self.replacer(s, db, &mut out);
            out
        };

        let mut line: Vec<Token> = db.term().get(format!("DC-SkillNodeDesc-{}", self.action_type).as_str());
        loop {
            let has_var = line.iter().any(|l| match l {
                Token::Var(_) => true,
                _ => false,
            });
            if !has_var {
                break;
            }

            let mut replaced = false;
            line = line.into_iter().flat_map(|l| match l {
                Token::Var(s) => {
                    let n = replacer(s.as_str());
                    if n.is_empty() {
                        vec![Token::Var(s.clone())]
                    } else {
                        replaced = true;

                        let mut out = vec![];
                        // out.push(Node::Text(format!("<{}:", s)));
                        out.extend(n);
                        // out.push(Node::Text(">".to_string()));
                        out
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
                "1" => vec![Token::Text(self.act_num.to_string())],
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

    pub fn rarity_range(&self) -> Range<i8> {
        let mut min = i8::MAX;
        let mut max = i8::MIN;
        for skill in self.iter() {
            if skill.rarity < min {
                min = skill.rarity;
            }
            if skill.rarity > max {
                max = skill.rarity;
            }
        }
        min..max+1
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

type Nodes = Vec<Token>;

trait ToDescs {
    fn map_var<F: Fn(&str) -> Vec<Token>>(&self, f: F) -> Vec<Token>;
}

impl ToDescs for Nodes {
    fn map_var<F: Fn(&str) -> Vec<Token>>(&self, f: F) -> Vec<Token> {
        self.iter().flat_map(|n| match n {
            Token::Var(s) => {
                let adds = f(s.as_str());
                if adds.is_empty() {
                    vec![n.clone()]
                } else {
                    let mut out = vec![];
                    // out.push(Node::Text(format!("<{}:", s)));
                    out.extend(adds);
                    // out.push(Node::Text(">".to_string()));
                    out
                }
            }
            n => vec![n.clone()],
        }).collect::<Vec<_>>()
    }
}
