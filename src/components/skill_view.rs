#![allow(non_snake_case)]

use dioxus::prelude::*;
use data::Database;
use data::skill::Skill;

#[component]
pub fn SkillView<'a>(cx: Scope<'a>, skill: &'a Skill) -> Element {
    let database = use_shared_state::<Database>(cx).unwrap().read();
    render! {
        div {
            h1 { database.tr(&skill.name()) }
        }
     }
}