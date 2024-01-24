use dioxus::prelude::*;
use dioxus_signals::Signal;
use fermi::use_read;

use crate::atoms::DATABASE;
use crate::components::SkillView;

#[component]
pub fn SkillDebugPage(cx: Scope) -> Element {
    let db = use_read(cx, &DATABASE);

    render! {
        for skill in db.skill.iter() {
            SkillView { skill: Signal::new(skill.clone()), debug: true }
        }
    }
}
