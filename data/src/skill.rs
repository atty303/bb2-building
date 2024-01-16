use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut, Range};

use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use term::TermRepository;
use token::{Token, Tokens};
use {Database, Sprite};

type SkillHash = u16;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AvroSchema, EnumString, Display,
)]
pub enum SkillCategory {
    #[strum(serialize = "0.Attack")]
    Attack,
    #[strum(serialize = "1.Summon")]
    Summon,
    #[strum(serialize = "2.Support")]
    Support,
    #[strum(serialize = "3.Survive")]
    Survive,
    #[strum(serialize = "4.Special")]
    Special,
    #[strum(serialize = "9.Enemy")]
    Enemy,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AvroSchema, EnumString, Display,
)]
pub enum AvoidType {
    #[strum(serialize = "")]
    None,
    A,
    C,
    LastHit,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AvroSchema, EnumString, Display,
)]
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

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AvroSchema, EnumString, Display,
)]
pub enum Target {
    SELF,
    TARGET,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AvroSchema, EnumString, Display,
)]
pub enum Reduce {
    #[strum(serialize = "")]
    None,
    P,
    M,
    V,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct Skill {
    pub hash: SkillHash,
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
    pub fn name(&self, terms: &TermRepository) -> String {
        format!("{}", terms.get(format!("NM-{}", self.modes[0].id).as_str()))
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
    pub fn name(&self, terms: &TermRepository) -> String {
        terms.get_fmt_str(&format_args!("NM-{}", self.id))
    }

    pub fn format(&self, db: &Database) -> Tokens {
        let mut tokens = Tokens(vec![]);

        let line1 = db
            .term()
            .get(if self.is_alt {
                "NM-SkillNodeDesc-ModeName-AltMode"
            } else {
                "NM-SkillNodeDesc-ModeName-Normal"
            })
            .map_var(|out, s| match s {
                "0" => {
                    if self.is_brave {
                        out.extend(db.term().get("NM-SkillNodeDesc-ModeName-ForBrave"))
                    } else {
                        out.push(Token::Empty);
                    }
                }
                _ => (),
            });
        tokens.extend(line1);
        tokens.push(Token::NewLine);

        for act in &self.acts {
            tokens.extend(act.format(db));
            tokens.push(Token::NewLine);
        }

        tokens.extend(db.term().get("WD-Cooldown"));
        tokens.push(Token::Text(format!(": {}", self.cooldown)));
        tokens.push(Token::NewLine);

        tokens.extend(db.term().get("WD-SkillPossRemain"));
        tokens.push(Token::Text(format!(
            ": -{}/{}",
            self.use_num, self.poss_num
        )));
        tokens.push(Token::NewLine);

        tokens
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
    pub fn format(&self, db: &Database) -> Tokens {
        let mut tokens = Tokens(vec![]);

        tokens.extend(
            db.term
                .get(format!("NM-SkillNodeDesc-ActTrigger-{}", self.act_trigger).as_str()),
        );
        tokens.push(Token::NewLine);

        for node in &self.nodes {
            tokens.extend(node.format(db));
            tokens.push(Token::NewLine);
        }
        tokens
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
    fn replacer(&self, name: &str, db: &Database, out: &mut Tokens) {
        match name {
            "lasthit" => match self.avoid_type {
                AvoidType::LastHit => out.extend(db.term().get("DC-SkillNodeDesc-LastHit")),
                _ => out.push(Token::Empty),
            },
            "t" => {
                let target = if self.target < 0 { 0 } else { self.target };
                out.extend(
                    db.term()
                        .get(format!("DC-SkillNodeDesc-TargetName-{}", target).as_str()),
                );
            }
            "tg" =>
            // TODO: param_key じゃないっぽい(-1)
            {
                out.extend(
                    db.term()
                        .get(format!("DC-SkillNodeDesc-TargetSkill-{}", self.param_key).as_str()),
                )
            }
            "dr" => out.extend(db.term().get("WD-DamageType-Direct")),
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
            "hit" => out.push(Token::Text(self.hit_rate.to_string())),
            "crit" => {
                if self.crit_rate == 0 || self.crit_rate == 100 {
                    out.push(Token::Empty)
                } else {
                    out.push(Token::NewLine);
                    out.push(Token::Text("　".to_string()));
                    out.extend(db.term().get("DC-SkillNodeDesc-CritRate").map_var(
                        |out, s| match s {
                            "0" => out.push(Token::Text(self.crit_rate.to_string())),
                            _ => (),
                        },
                    ));
                }
            }
            "rd" => match self.reduce {
                Reduce::None => out.push(Token::Empty),
                Reduce::P | Reduce::M | Reduce::V => {
                    out.push(Token::NewLine);
                    out.push(Token::Text("　".to_string()));
                    out.extend(
                        db.term()
                            .get(format!("DC-SkillNodeDesc-Reduce-{}", self.reduce).as_str()),
                    );
                }
            },
            "inc" => {
                if self.inc_relate.is_empty() {
                    out.push(Token::Empty)
                } else {
                    out.push(Token::NewLine);
                    out.push(Token::Text("　".to_string()));
                    let pair = self.inc_relate.split(':').collect::<Vec<_>>();
                    let key = pair[0];
                    match key {
                        "CritRate" => out.extend(db.term().get("DC-SkillNodeDesc-AboutIncPower")),
                        _ => (),
                    }
                }
            }
            "irt" => match self.inc_target {
                Target::SELF => out.extend(db.term().get("DC-SkillNodeDesc-TargetName-0")),
                Target::TARGET => out.extend(db.term().get("DC-SkillNodeDesc-TargetName-1")),
            },
            "irf" => {
                let pair = self.inc_relate.split(':').collect::<Vec<_>>();
                let key = pair[0];
                out.extend(db.term().get(format!("NM-{}", key).as_str()));
            }
            "ipw" => out.push(Token::Text(format!("{}", self.inc_power))),
            "power" => out.extend(db.term().get("DC-SkillNodeDesc-AboutPower")),
            "pwd" =>
            // rt,rf,pw
            {
                out.extend(db.term().get("DC-SkillNodeDesc-AboutPowerDtl"))
            }
            "rt" => match self.inc_target {
                Target::SELF => out.extend(db.term().get("DC-SkillNodeDesc-TargetName-0")),
                Target::TARGET => out.extend(db.term().get("DC-SkillNodeDesc-TargetName-1")),
            },
            "rf" => {
                if self.relate.contains('/') {
                    let mut it = self.relate.split('/');
                    let or = [it.next().unwrap(), it.next().unwrap()]
                        .iter()
                        .map(|s| {
                            let n = &s[3..4];
                            db.term().get(format!("NM-MainParam:{}", n).as_str())
                        })
                        .collect::<Vec<_>>();
                    out.extend(or[0].clone());
                    out.extend(db.term().get("WD-Relate-Or"));
                    out.extend(or[1].clone());
                } else {
                    let pair = self.relate.split(':').collect::<Vec<_>>();
                    let key = pair[0];
                    out.extend(db.term().get(format!("NM-{}", key).as_str()));
                }
            }
            "pw" => out.push(Token::Text(format!("{}", self.power))),
            "last" => {
                if self.state_last.room >= 0 {
                    out.push(Token::NewLine);
                    out.push(Token::Text("　".to_string()));
                    out.extend(
                        db.term()
                            .get("DC-SkillNodeDesc-LastCombine")
                            .map_var(|out, s| match s {
                                "0" => {
                                    let t = db.term().get("DC-SkillNodeDesc-LastRoom").map_var(
                                        |out, s| match s {
                                            "0" => {
                                                out.push(Token::Text(
                                                    self.state_last.room.to_string(),
                                                ));
                                            }
                                            _ => (),
                                        },
                                    );
                                    out.extend(t);
                                }
                                _ => (),
                            }),
                    );
                } else if self.state_last.f1 >= 0 {
                    out.push(Token::Error("state_last.f1".to_string()));
                } else if self.state_last.f2 >= 0 {
                    out.push(Token::NewLine);
                    out.push(Token::Text("　".to_string()));
                    out.extend(
                        db.term()
                            .get("DC-SkillNodeDesc-LastCombine")
                            .map_var(|out, s| match s {
                                "0" => {
                                    let t = db.term().get("DC-SkillNodeDesc-LastTurn").map_var(
                                        |out, s| match s {
                                            "0" => {
                                                out.push(Token::Text(
                                                    self.state_last.f2.to_string(),
                                                ));
                                            }
                                            _ => (),
                                        },
                                    );
                                    out.extend(t);
                                }
                                _ => (),
                            }),
                    );
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
            "srpw" => {
                if let Some(state_row_id) = &self.state_row_id {
                    if let Some(state) = db.state.get(state_row_id) {
                        let text = state.format.replace("{v}", &format!("{}", self.power));
                        out.push(Token::Text(text));
                    } else {
                        out.push(Token::Error(format!("state not found: {}", state_row_id)));
                    }
                }
            }
            "stpw" => out.push(Token::Empty), // TODO:
            "md" => {
                if self.action_type == "AltMode" {
                    if self.power == 0 {
                        out.extend(db.term().get("WD-SkillAltModeName-0"));
                    } else if self.power == 1 {
                        out.extend(db.term().get("WD-SkillAltModeName-1"));
                    } else {
                        out.push(Token::Error(format!("invalid power {}", self.power)));
                    }
                } else {
                    out.push(Token::Error(format!(
                        "invalid action_type {}",
                        self.action_type
                    )));
                }
            }

            _ => (),
        }
    }

    pub fn format(&self, db: &Database) -> Tokens {
        let replacer = |s: &str| {
            let mut out = Tokens(vec![]);
            self.replacer(s, db, &mut out);
            out
        };

        let mut line: Tokens = db
            .term()
            .get(format!("DC-SkillNodeDesc-{}", self.action_type).as_str());
        loop {
            if !line.has_var() {
                break;
            }

            let mut replaced = false;
            line = Tokens(
                line.0
                    .into_iter()
                    .flat_map(|l| match l {
                        Token::Var(s) => {
                            let n = replacer(s.as_str());
                            if n.is_empty() {
                                vec![Token::Var(s.clone())]
                            } else {
                                replaced = true;

                                let mut out = vec![];
                                out.extend(n.0);
                                out
                            }
                        }
                        _ => vec![l.clone()],
                    })
                    .collect::<Vec<_>>(),
            );
            if !replaced {
                break;
            }
        }

        let line = if self.act_num == 1 {
            line
        } else {
            db.term()
                .get("DC-SkillNodeDesc-MultipleCase")
                .map_var(|out, s| match s {
                    "0" => out.extend(line.clone()),
                    "1" => out.push(Token::Text(self.act_num.to_string())),
                    _ => (),
                })
        };

        line
    }
}

#[derive(Clone, Default)]
pub struct SkillRepository {
    inner: HashMap<SkillHash, Skill>,
    order: Vec<SkillHash>,
}

impl SkillRepository {
    pub fn write<'a, W: Write, I: Iterator<Item = &'a Skill>>(
        avro_write: W,
        skills: I,
    ) -> Result<(), apache_avro::Error> {
        let schema = Skill::get_schema();
        let mut writer =
            apache_avro::Writer::with_codec(&schema, avro_write, apache_avro::Codec::Deflate);
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
        min..max + 1
    }
}

impl Deref for SkillRepository {
    type Target = HashMap<SkillHash, Skill>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for SkillRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
