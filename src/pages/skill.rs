use dioxus::prelude::*;
use dioxus_router::prelude::{use_navigator, Link};
use fermi::use_read_rc;

use data::skill::Skill;
pub use search::SkillSearchPage;

use crate::atoms::DATABASE;
use crate::components::{Rarity, SkillView, SpriteIcon};
use crate::hooks::use_search_skill;
use crate::pages::Route;

mod search;

#[component]
pub fn SkillListPage(cx: Scope) -> Element {
    let db = use_read_rc(cx, &DATABASE);
    let rarities = db.skill.rarity_range();

    let search = use_search_skill(cx);

    render! {
        div {
            class: "text-sm breadcrumbs",
            ul {
                li { "Home" }
                li { "Skill" }
            }
        }

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

        if true {
            div { class: "flex flex-wrap gap-2",
                {search.results.read().iter().map(|hash| {
            rsx! {
                SkillView { skill_hash: *hash }
            }
        })}
            }
        } else {
            div { class: "p-2 divide-y",
                for rarity in rarities {
                    div { class: "py-4",
                        h2 { class: "mb-2",
                            Rarity { rarity: rarity }
                        }
                        div { class: "flex flex-wrap w-fit gap-2",
                            {search.results.read().iter()/*.filter(|s| s.rarity == rarity)*/.map(|hash| {
                                rsx! {
                                    SkillView { skill_hash: *hash }
                                }
                            })}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SkillLink<'a>(cx: Scope, skill: &'a Skill) -> Element<'a> {
    render! {
        span { class: "inline-block",
            title: "{skill.name}",
            Link { class: "inline-block hover:bg-primary border-primary border-solid border-2 rounded-md p-1",
                to: Route::SkillPage { skill_id: skill.id.clone() },
                SpriteIcon { class: "rounded-md", sprite: &skill.modes[0].icon, size: 96 }
            }

        }
    }
}

#[component]
pub fn SkillPage(cx: Scope, skill_id: String) -> Element {
    let db = use_read_rc(cx, &DATABASE);

    db.skill
        .values()
        .find(|s| &s.id == skill_id)
        .map(|skill| {
            render! {
                div {
                    class: "text-sm breadcrumbs",
                    ul {
                        li { "Home" }
                        li { "Skill" }
                        li { "{skill.name}" }
                    }
                }

                SkillView { skill_hash: skill.hash }
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
