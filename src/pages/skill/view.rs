use dioxus::prelude::*;
use fermi::use_read;

use crate::atoms::DATABASE;
use crate::components::SkillView;

#[component]
pub fn SkillPage(cx: Scope, skill_id: String) -> Element {
    let db = use_read(cx, &DATABASE);

    db.skill
        .values()
        .find(|s| &s.id == skill_id)
        .map(|skill| {
            render! {
                div { class: "text-sm breadcrumbs",
                    ul {
                        li { "Home" }
                        li { "Skill" }
                        li { "{skill.name}" }
                    }
                }

                SkillView { skill: skill }
            }
        })
        .unwrap_or_else(|| {
            render! {
                div {
                    "Skill not found"
                }
            }
        })
}
