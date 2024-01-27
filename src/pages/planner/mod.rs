use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use std::fmt::Display;
use std::str::FromStr;

use serde::de::Error;
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
    type Err = rmp_serde::decode::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = BASE64_URL_SAFE_NO_PAD
            .decode(s.as_bytes())
            .map_err(|_| rmp_serde::decode::Error::custom("invalid base64"))?;
        rmp_serde::decode::from_slice(&bytes)
    }
}

impl Display for PlannerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match rmp_serde::to_vec(self) {
            Ok(bytes) => {
                let s = BASE64_URL_SAFE_NO_PAD.encode(&bytes);
                f.write_str(&s)
            }
            Err(_) => Err(std::fmt::Error),
        }
    }
}
