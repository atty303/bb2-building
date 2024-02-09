use dioxus::prelude::*;

use crate::components::SkillView;
use crate::global::DATABASE;

#[component]
pub fn SkillDebugPage() -> Element {
    rsx! {
        for skill in DATABASE().skill.iter() {
            SkillView { skill: Signal::new(skill.clone()), debug: true }
        }
    }
}
