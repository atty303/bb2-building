use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut, Range};

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use term::TermRepository;
use token::{Token, Tokens};
use {Database, Sprite};

type SkillHash = u16;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum AvoidType {
    #[strum(serialize = "")]
    None,
    A,
    C,
    LastHit,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum Target {
    SELF,
    TARGET,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum Reduce {
    #[strum(serialize = "")]
    None,
    P,
    M,
    V,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, Display)]
pub enum ActTrigger {
    OnUse,
    TurnStart,
    Passive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
            tokens.extend(node.description.clone());
            tokens.push(Token::NewLine);
        }
        tokens
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    pub description: Tokens,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateLast {
    pub f1: i8,
    pub f2: i8, // TODO: turn
    pub f3: i8,
    pub room: i8,
    pub f5: i8,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct SkillRepository {
    inner: HashMap<SkillHash, Skill>,
    order: Vec<SkillHash>,
}

impl SkillRepository {
    pub fn from_vec(items: Vec<Skill>) -> Self {
        let mut inner = HashMap::new();
        let mut order = vec![];
        for item in items {
            let hash = item.hash;
            inner.insert(hash, item);
            order.push(hash);
        }
        Self { inner, order }
    }

    pub fn read<R: Read>(read: R) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::decode::from_read(read)
    }

    pub fn write<W: Write>(&self, write: &mut W) -> Result<(), rmp_serde::encode::Error> {
        rmp_serde::encode::write(write, self)
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
