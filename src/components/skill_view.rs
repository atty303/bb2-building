#![allow(non_snake_case)]

use dioxus::prelude::*;

use data::Database;
use crate::components::sprite::Sprite;

#[component]
pub fn SkillView<'a>(cx: Scope<'a>, skill: &'a data::skill::Skill) -> Element {
    let database = use_shared_state::<Database>(cx).unwrap().read();
    render! {
        div {
            class: "flex flex-col bg-base-300 text-base-content rounded-md p-2 my-2",
            div {
                class: "flex flex-row items-center gap-2",
                Sprite { sprite: &skill.modes[0].icon }
                span {
                    title: skill.id.as_str(),
                    database.tr(&skill.name())
                }
            }
            ul {
                class: "flex flex-row gap-2 p-2",
                for mode in &skill.modes {
                    li {
                        SkillMode { mode: mode }
                    }
                }
            }
        }
     }
}

#[component]
pub fn SkillMode<'a>(cx: Scope<'a>, mode: &'a data::skill::SkillMode) -> Element {
    let database = use_shared_state::<Database>(cx).unwrap();

    render! {
        div {
            class: "flex flex-col gap-2 bg-base-200 text-base-content rounded-md p-2",
            div {
                class: "flex flex-row items-center gap-2",
                Sprite { sprite: &mode.icon }
                div {
                    database.read().tr(&mode.name())
                }
            }
            for act in &mode.acts {
                ul {
                    for node in &act.nodes {
                        li {
                            &node.format(&database.read()).as_str()
                        }
                    }
                }
            }
        }
    }
}
