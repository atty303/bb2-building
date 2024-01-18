use crate::components::SkillView;
use crate::hooks::use_search_skill;
use dioxus::prelude::*;

#[component]
pub fn SkillSearchPage<'a>(cx: Scope<'a>) -> Element {
    let search = use_search_skill(cx);

    render! {
        div {
            input {
                class: "input input-bordered w-full max-w-xs",
                r#type: "text",
                placeholder: "Search",
                oninput: move |e| {
                    let q = e.data.value();
                    search.query.set(q);
                }
            }
        }

        {search.results.read().iter().map(|hash| {
            rsx! {
                SkillView { skill_hash: *hash }
            }
        })}
    }
}
