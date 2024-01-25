use dioxus::prelude::*;

use crate::components::SkillView;
use crate::global::DATABASE;

#[component]
pub fn SkillPage(skill_id: String) -> Element {
    DATABASE()
        .skill
        .iter()
        .find(|s| s.id == skill_id)
        .map(|skill| {
            rsx! {
                div { class: "text-sm breadcrumbs",
                    ul {
                        li { "Home" }
                        li { "Skill" }
                        li { "{skill.name}" }
                    }
                }

                SkillView { skill: Signal::new(skill.clone()) }
            }
        })
        .unwrap_or_else(|| {
            rsx! {
                div {
                    "Skill not found"
                }
            }
        })
}
