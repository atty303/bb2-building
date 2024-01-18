use crate::atoms::{DATABASE, SEARCH_INDEX};
use crate::components::SkillView;
use data::skill::{Skill, SkillHash};
use dioxus::prelude::*;
use fermi::{use_atom_state, use_read, use_read_rc};

#[component]
pub fn SkillSearchPage<'a>(cx: Scope<'a>) -> Element {
    let db = use_read(cx, &DATABASE);
    let index = use_atom_state(cx, &SEARCH_INDEX);

    let query = use_ref(cx, || String::new());
    let results = use_ref(cx, || Vec::<SkillHash>::new());

    use_effect(cx, (db, query), move |(db, query)| {
        to_owned![index, results];
        async move {
            let q = query.read().clone();
            let indexes = if q.is_empty() {
                db.skill.iter().map(|skill| skill.hash).collect()
            } else {
                index.skill.search(&*q).iter().map(|hash| **hash).collect()
            };
            results.set(indexes);
        }
    });

    let skills = results.read();
    render! {
        div {
            input {
                class: "input input-bordered w-full max-w-xs",
                r#type: "text",
                placeholder: "Search",
                oninput: move |e| {
                    let q = e.data.value();
                    query.set(q);
                }
            }
        }

        {results.read().iter().map(|hash| {
            rsx! {
                SkillView { skill_hash: *hash }
            }
        })}
    }
}
