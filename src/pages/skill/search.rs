use crate::atoms::{DATABASE, SEARCH_INDEX};
use crate::components::SkillView;
use data::skill::SkillHash;
use dioxus::prelude::*;
use fermi::{use_atom_state, use_read_rc};

#[component]
pub fn SkillSearchPage(cx: Scope) -> Element {
    let db = use_read_rc(cx, &DATABASE);
    let index = use_atom_state(cx, &SEARCH_INDEX);

    let results = use_ref(cx, || Vec::<SkillHash>::new());

    render! {
        div {
            input {
                class: "input input-bordered w-full max-w-xs",
                r#type: "text",
                placeholder: "Search",
                oninput: move |e| {
                    let query = e.data.value();
                    let indexes = index.skill.search(&query);
                    results.set(indexes.iter().map(|hash| **hash).collect::<Vec<_>>());
                }
            }
        }

        {results.read().iter().map(|hash| {
            rsx! {
                SkillView { skill: {db.skill.get(&hash).unwrap()} }
            }
        })}
    }
}
