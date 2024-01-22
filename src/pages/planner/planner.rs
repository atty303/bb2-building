use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlannerState {}

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

#[component]
pub fn PlannerPage(cx: Scope, state: PlannerState) -> Element {
    render! {
        h1 { "BB2B" }
    }
}
