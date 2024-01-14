#![allow(non_snake_case)]

use dioxus::prelude::*;

use data::Database;
use data::skill::Skill;
use crate::components::sprite::Sprite;

#[component]
pub fn SkillView<'a>(cx: Scope<'a>, skill: &'a Skill) -> Element {
    let database = use_shared_state::<Database>(cx).unwrap().read();
    render! {
        div {
            h1 {
                Sprite { sprite: &skill.modes[0].icon }
                database.tr(&skill.name())
            }
            ul {
                li {
                }
            }
        }
     }
}