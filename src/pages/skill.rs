use std::collections::HashSet;

use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use dioxus_signals::{ReadOnlySignal, Signal};
use fermi::use_read_rc;

use data::skill::Skill;

use crate::atoms::DATABASE;
use crate::components::{Rarity, SkillView, SpriteIcon};
use crate::hooks::use_search_skill;
use crate::pages::Route;

#[component]
pub fn SkillListPage(cx: Scope) -> Element {
    let search = use_search_skill(cx);

    let rarities = search
        .results
        .read()
        .iter()
        .map(|s| s.read().rarity)
        .collect::<HashSet<_>>();
    let mut rarities = rarities.iter().collect::<Vec<_>>();
    rarities.sort_unstable();

    render! {
        div { class: "text-sm breadcrumbs",
            ul {
                li { "Home" }
                li { "Skill" }
            }
        }

        div {
            input { class: "input input-bordered w-full max-w-xs",
                r#type: "text",
                placeholder: "Search",
                oninput: move |e| {
                    let q = e.data.value();
                    search.query.set(q);
                }
            }
        }

        if false {
            div { class: "flex flex-wrap gap-2",
                for skill in search.results.read().iter() {
                    SkillLink { skill: *skill }
                }
            }
        } else {
            div { class: "p-2 divide-y",
                for rarity in rarities {
                    div { class: "py-4",
                        h2 { class: "mb-2",
                            Rarity { rarity: *rarity }
                        }
                        div { class: "flex flex-wrap w-fit gap-2",
                            for skill in search.results.read().iter().filter(|s| s.read().rarity == *rarity) {
                                SkillLink { skill: *skill }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SkillLink(cx: Scope, skill: Signal<Skill>) -> Element {
    render! {
        span { class: "inline-block",
            title: "{skill.read().name}",
            Link { class: "inline-block hover:bg-primary border-primary border-solid border-2 rounded-md p-1",
                to: Route::SkillPage { skill_id: skill.read().id.clone() },
                SpriteIcon { class: "rounded-md", sprite: ReadOnlySignal::new(Signal::new(skill.read().modes[0].icon.clone())), size: 96 }
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
