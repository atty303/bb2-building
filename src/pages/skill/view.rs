use dioxus::prelude::*;

use crate::components::SkillView;
use crate::global::DATABASE;
use crate::Language;

#[component]
pub fn SkillPage(language: Language, skill_id: String) -> Element {
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

                SkillView { language, skill: Signal::new(skill.clone()) }
            }
        })
        .unwrap_or_else(|| {
            rsx! { div { "Skill not found" } }
        })
}
