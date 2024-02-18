use dioxus::prelude::*;

use crate::components::SkillView;
use crate::global::DATABASE;
use crate::Language;

#[component]
pub fn SkillDebugPage(language: Language) -> Element {
    rsx! {
        for skill in DATABASE().skill.iter() {
            SkillView { language: language.clone(), skill: Signal::new(skill.clone()), debug: true }
        }
    }
}
