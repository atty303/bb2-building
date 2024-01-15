use dioxus::prelude::*;
use fermi::use_read_rc;

use crate::atoms::DATABASE;
use crate::components::skill_view::SkillView;

#[component]
pub fn SkillListPage(cx: Scope) -> Element {
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
