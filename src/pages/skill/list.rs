use std::fmt::Display;
use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_signals::{use_signal, Signal};
use fermi::use_read;
use serde::{Deserialize, Serialize};

use data::skill::Skill;

use crate::atoms::DATABASE;
use crate::components::{SkillView, SpriteIcon};
use crate::hooks::{use_modal, use_search_skill, ModalDialogProps};
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
    let selected = use_signal(cx, || None);
    render! {
        div { class: "text-sm breadcrumbs",
            ul {
                li { "Home" }
                li { "Skill" }
            }
        }

        SkillList {
            query: query.query,
            on_search: move |q: String| {
                let router = dioxus_router::router();
                router.replace(Route::SkillListPage {
                    query: SkillListQuery {
                        query: Signal::new(q.clone()),
                    },
                });
            },
            selected: selected,
        }
    }
}

#[component]
pub fn SkillList<'a>(
    cx: Scope<'a>,
    query: Signal<String>,
    on_search: EventHandler<'a, String>,
    selected: Signal<Option<Signal<Skill>>>,
) -> Element {
    let db = use_read(cx, &DATABASE);

    let detail_modal = use_modal(cx, "max-w-full h-full".to_string());

    let search = use_search_skill(cx);
    if *search.query.peek() != *query.peek() {
        search.query.set(query.peek().clone());
    }

    render! {
        div {
            div { class: "flex flex-row items-center gap-4",
                div { class: "relative flex-grow",
                    input { class: "input input-bordered input-primary w-full",
                        r#type: "text",
                        placeholder: "Search skills...",
                        autofocus: true,
                        value: "{query}",
                        oninput: move |e| {
                            let q = e.data.value();
                            search.query.set(q.clone());
                            on_search.call(q.clone());
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
                        on_click: move |skill| {
                            selected.set(Some(skill));
                            detail_modal.show_modal(skill, move |_| {});
                        },
                    }
                }
            }

        }
        {detail_modal.component(cx, DetailModal)}
    }
}

pub fn DetailModal<'a>(cx: Scope<'a, ModalDialogProps<'a, Skill, i32>>) -> Element {
    if let Some(skill) = *cx.props.props.read() {
        render! {
            SkillView { skill: skill }
        }
    } else {
        None
    }
}

#[component]
fn SkillLink<'a>(
    cx: Scope<'a>,
    skill: Signal<Skill>,
    on_click: EventHandler<'a, Signal<Skill>>,
) -> Element {
    #[component]
    fn SkillLinkInnerIcon<'a>(
        cx: Scope,
        skill: Signal<Skill>,
        class: &'a str,
        size: i32,
        on_click: EventHandler<'a, Signal<Skill>>,
    ) -> Element {
        render! {
            button { class: "hover:bg-primary border-primary border-solid border-2 rounded-md p-1 {class}",
                onclick: move |_| on_click.call(skill.clone()),
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
        SkillLinkInnerIcon { skill: skill.clone(), class: "inline-block md:hidden", size: 56, on_click: move |e| on_click.call(e) }
        SkillLinkInnerIcon { skill: skill.clone(), class: "hidden md:inline-block", size: 96, on_click: move |e| on_click.call(e) }
    }
}
