use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut, Range};

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use sprite::Sprite;
use token::{Token, Tokens};

type SkillHash = u16;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, Display)]
pub enum ActTrigger {
    OnUse,
    TurnStart,
    Passive,
}

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
    // extra fields
    pub name: String,
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
    // extra fields
    pub name: String,
    pub description_head: Tokens,
    pub description_body: Option<Tokens>,
    pub description_tail: Tokens,
    pub poss_num: i8, // copy from Skill
}

impl SkillMode {
    pub fn format(&self) -> Tokens {
        let mut tokens = self.description_head.clone();
        let out = &mut tokens;
        Token::NewLine.write(out);

        if let Some(body) = &self.description_body {
            body.write(out);
            Token::NewLine.write(out);
        } else {
            for act in &self.acts {
                act.format().write(out);
                Token::NewLine.write(out);
            }
        }

        Token::NewLine.write(out);
        self.description_tail.write(out);
        tokens
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Act {
    pub id: String,
    pub act_trigger: ActTrigger,
    pub nodes: Vec<ActNode>,
    // extra
    pub description: Tokens,
}

impl Act {
    pub fn format(&self) -> Tokens {
        let mut tokens = Tokens::new();
        let out = &mut tokens;

        self.description.write(out);
        Token::NewLine.write(out);

        let mut first = true;
        for node in self.nodes.iter().filter(|n| !n.description.is_empty()) {
            if first {
                first = false;
            } else {
                Token::NewLine.write(out);
            }
            node.format().write(out);
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
    pub hit_rate: u16,
    pub avoid_type: AvoidType,
    pub relate_target: Target,
    pub relate: String,
    pub power: u32,
    pub reduce: Reduce,
    pub inc_target: Target,
    pub inc_relate: String,
    pub inc_power: u16,
    pub act_num: u8,
    pub crit_rate: u16,
    // extra
    pub description: Tokens,
}

impl ActNode {
    pub fn format(&self) -> Tokens {
        self.description.clone()
    }
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
