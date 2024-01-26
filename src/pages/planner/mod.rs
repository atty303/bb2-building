use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use data::{RuneHash, SkillHash};

mod edit;

pub use edit::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlannerState {
    build: BuildState,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
struct BuildState {
    slots: [SlotState; 5],
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
struct SlotState {
    skill: Option<SkillHash>,
    runes: [RuneHash; 5],
}

impl FromStr for PlannerState {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for PlannerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(s) => f.write_str(&s),
            Err(_) => Err(std::fmt::Error),
        }
    }
}
