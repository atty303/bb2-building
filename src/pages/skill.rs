use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use fermi::use_read_rc;

use crate::atoms::DATABASE;
use crate::components::skill_view::SkillView;
use crate::components::sprite::{Sprite, SpriteIcon};
use crate::pages::Route;

#[component]
pub fn SkillListPage(cx: Scope) -> Element {
    let db = use_read_rc(cx, &DATABASE);

    render! {
        div {
            class: "text-sm breadcrumbs",
            ul {
                li { "Home" }
                li { "Skill" }
            }
        }

        Link {
            to: Route::SkillSearchPage {},
            "Search"
        }

        div { class: "grid grid-cols-5 w-fit gap-2",

            for skill in db.skill.iter().filter(|s| s.in_dictionary) {
                Link { class: "hover:bg-primary border-primary border-solid border-2 rounded-md p-1",
                    to: Route::SkillPage { skill_id: skill.id.clone() },
                    SpriteIcon { class: "rounded-md", sprite: &skill.modes[0].icon, size: 96 }
                }

            }
        }
    }
}

#[component]
pub fn SkillSearchPage(cx: Scope) -> Element {
    let db = use_read_rc(cx, &DATABASE);

    render! {
        for skill in db.skill.iter().filter(|s| s.in_dictionary) {
            SkillView { skill: &skill }
        }
    }
}

#[component]
pub fn SkillPage(cx: Scope, skill_id: String) -> Element {
    let db = use_read_rc(cx, &DATABASE);

    db.skill.values().find(|s| &s.id == skill_id).map(|skill| {
        render! {
            div {
                class: "text-sm breadcrumbs",
                ul {
                    li { "Home" }
                    li { "Skill" }
                    li { skill.name(db.term()) }
                }
            }

            SkillView { skill: &skill }
        }
    }).unwrap_or_else(|| {
        render! {
            div {
                "Skill not found"
            }
        }
    })
}
