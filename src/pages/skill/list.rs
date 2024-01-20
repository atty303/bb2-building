use std::fmt::Display;
use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_signals::{use_signal, Signal};
use fermi::use_read;
use serde::{Deserialize, Serialize};

use data::skill::Skill;

use crate::atoms::DATABASE;
use crate::components::{SkillView, SpriteIcon};
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

    let selected = use_signal(cx, || None::<Skill>);
    let bottom_height = use_signal(cx, || 0);

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
            div { class: "badge badge-accent badge-lg gap-1 text-xs",
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

        div { class: "flex flex-wrap gap-2 mt-4",
            for skill in search.results.read().iter() {
                SkillLink {
                    skill: *skill,
                    selected: selected.clone(),
                }
            }
        }

        if selected.read().is_some() {
            div {
                div {
                    // TODO: Footer の高さの余白が出来てしまうので引く
                    height: "{bottom_height}px",
                }
                div { class: "fixed bottom-0 left-0 z-50 w-full h-2/3 sm:h-1/2 bg-base-100 border-t border-neutral overflow-y-auto shadow-inner",
                    onmounted: move |e| {
                        async move {
                            let  _ = e.data.get_client_rect().await.map(|rect| {
                                bottom_height.set(rect.size.height as i32);
                            });
                        }
                    },
                    if let Some(skill) = selected.read().as_ref() {
                        SkillView { skill: Signal::new(skill.clone()) }
                    }
                }
            }
        }
    }
}

#[component]
fn SkillLink(cx: Scope, skill: Signal<Skill>, selected: Signal<Option<Skill>>) -> Element {
    #[component]
    fn SkillLinkInnerIcon<'a>(
        cx: Scope,
        skill: Signal<Skill>,
        class: &'a str,
        size: i32,
    ) -> Element {
        render! {
            span { class: "hover:bg-primary border-primary border-solid border-2 rounded-md p-1 {class}",
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

    let active = use_signal(cx, || false);

    render! {
        button { class: "inline-block",
            onclick: move |_| selected.set(Some(skill.read().clone())),
            SkillLinkInnerIcon { skill: skill.clone(), class: "inline-block md:hidden", size: 56 }
            SkillLinkInnerIcon { skill: skill.clone(), class: "hidden md:inline-block", size: 96 }
        }
    }
}
