#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use fermi::use_read;

use data::Database;
use crate::atoms::DATABASE;

use crate::components::sprite::Sprite;
use crate::pages::Route;

#[component]
pub fn SkillView<'a>(cx: Scope<'a>, skill: &'a data::skill::Skill) -> Element {
    let database = use_read(cx, &DATABASE);
    render! {
        div {
            class: "flex flex-col border-solid border border-base-300 rounded-md my-2",
            div {
                class: "flex flex-row items-center gap-2 bg-base-300 text-base-content p-2",
                Sprite { sprite: &skill.modes[0].icon }
                span {
                    class: "flex-grow",
                    Link {
                        class: "text-primary hover:underline cursor-pointer",
                        to: Route::SkillPage { skill_id: skill.id.clone() },
                        database.tr(&skill.name())
                    }
                }
                span {
                    Rarity { rarity: skill.rarity }
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
    let database = use_read(cx, &DATABASE);

    let desc = &mode.format(database)
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\n", "<br>");

    render! {
        div {
            class: "flex flex-col gap-2 bg-base-200 text-base-content rounded-md p-2",
            div {
                class: "flex flex-row items-center gap-2",
                Sprite { sprite: &mode.icon }
                div {
                    database.tr(&mode.name())
                }
            }
            span {
                dangerous_inner_html: "{desc}",
            }
        }
    }
}

#[component]
fn Rarity(cx: Scope, rarity: i8) -> Element {
    let db = use_read(cx, &DATABASE);
    let color = db.tr_str(format!("CLR-Star-Rarity-{}", rarity));
    render! {
        div {
            class: "flex flex-row",
            color: "#{color}",
            for _ in 0..(*rarity) {
                svg {
                    class: "w-4 h-4",
                    view_box: "0 0 24 24",
                    fill: "currentColor",
                    path {
                        fill_rule: "evenodd",
                        clip_rule: "evenodd",
                        d: "M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.006 5.404.434c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.434 2.082-5.005Z",
                    }
                }
            }
        }
    }
}