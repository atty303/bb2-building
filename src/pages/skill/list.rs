use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use dioxus_signals::Signal;
use fermi::use_read;
use serde::{Deserialize, Serialize};

use data::skill::Skill;

use crate::atoms::DATABASE;
use crate::components::{Rarity, SpriteIcon};
use crate::hooks::use_search_skill;
use crate::pages::Route;

#[derive(Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SkillListQuery {
    query: Signal<String>,
}

impl FromStr for SkillListQuery {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for SkillListQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(s) => f.write_str(&s),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

#[component]
pub fn SkillListPage(cx: Scope, query: SkillListQuery) -> Element {
    let db = use_read(cx, &DATABASE);

    let search = use_search_skill(cx);
    if *search.query.peek() != *query.query.peek() {
        search.query.set(query.query.peek().clone());
    }

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

        div { class: "flex flex-row items-center gap-4",
            div { class: "relative flex-grow",
                input { class: "input input-bordered input-primary w-full",
                    r#type: "text",
                    placeholder: "Search skills...",
                    autofocus: true,
                    value: "{query.query}",
                    oninput: move |e| {
                        let q = e.data.value();
                        search.query.set(q.clone());
                        let router = dioxus_router::router();
                        router.replace(Route::SkillListPage {
                            query: SkillListQuery {
                                query: Signal::new(q.clone()),
                            },
                        });
                    }
                }
                // button { class: "absolute inset-y-0 right-0 flex items-center pr-2",
                //     onclick: move |_| {
                //         search.query.set("".to_string());
                //     },
                //     Icon {
                //         svg: r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" /></svg>"#,
                //     }
                // }
            }
            div { class: "badge badge-accent badge-lg gap-1",
                span { class: "font-bold",
                    "{search.results.read().len()}"
                }
                span {
                    "of"
                }
                span { class: "font-bold",
                    "{db.skill.iter().count()}"
                }
            }
        }

        if true {
            div { class: "flex flex-wrap gap-2 mt-4",
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
    #[component]
    fn SkillLinkInnerIcon<'a>(
        cx: Scope,
        skill: Signal<Skill>,
        class: &'a str,
        size: i32,
    ) -> Element {
        render! {
            Link { class: "hover:bg-primary border-primary border-solid border-2 rounded-md p-1 {class}",
                to: Route::SkillPage { skill_id: skill.read().id.clone() },
                span { class: "relative",
                    SpriteIcon { class: "rounded-md",
                        sprite: Signal::new(skill.read().modes[0].icon.clone()),
                        size: *size,
                    }
                    span { class: "absolute right-0 bg-black/50 text-white text-xs px-1 text-right",
                        "{skill.read().name}"
                    }
                }
            }
        }
    }

    render! {
        span { class: "inline-block",
            SkillLinkInnerIcon { skill: skill.clone(), class: "inline-block md:hidden", size: 56 }
            SkillLinkInnerIcon { skill: skill.clone(), class: "hidden md:inline-block", size: 96 }
        }
    }
}
