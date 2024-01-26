use std::fmt::Display;
use std::str::FromStr;

use dioxus::prelude::*;
use dioxus::router::router;
use dioxus_headlessui::dialog::{Dialog, DialogPanel};
use serde::{Deserialize, Serialize};

use crate::components::SkillView;
use data::skill::Skill;

use crate::global::DATABASE;
use crate::hooks::use_search_skill;
use crate::pages::Route;
use crate::ui::SpriteIcon;

#[derive(Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SkillListState {
    query: Signal<String>,
}

impl FromStr for SkillListState {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for SkillListState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(s) => f.write_str(&s),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

#[component]
pub fn SkillListPage(state: SkillListState) -> Element {
    rsx! {
        div { class: "text-sm breadcrumbs",
            ul {
                li { "Home" }
                li { "Skill" }
            }
        }

        SkillList {
            query: state.query,
            on_search: move |q: String| {
                router().replace(Route::SkillListPage {
                    state: SkillListState {
                        query: Signal::new(q.clone()),
                    },
                });
            },
        }
    }
}

#[component]
pub fn SkillList(query: Signal<String>, on_search: EventHandler<String>) -> Element {
    let detail_open = use_signal(|| false);
    let detail_skill = use_signal(|| None);

    let search = use_search_skill();
    if *search.query.peek() != *query.peek() {
        *search.query.write() = query.peek().clone();
    }

    rsx! {
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
                            *search.query.write() = q.clone();
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
                        "{DATABASE().skill.iter().count()}"
                    }
                }
            }

            div { class: "flex flex-wrap gap-2 mt-4",
                for skill in search.results.read().iter() {
                    SkillLink {
                        skill: *skill,
                        on_click: move |skill: Signal<Skill>| {
                            *detail_open.write() = true;
                            *detail_skill.write() = Some(skill.clone());
                        },
                    }
                }
            }
        }

        DetailDialog {
            open: detail_open,
            maybe_skill: detail_skill,
        }
    }
}

#[component]
pub fn DetailDialog(open: Signal<bool>, maybe_skill: Signal<Option<Signal<Skill>>>) -> Element {
    if let Some(skill) = maybe_skill() {
        rsx! {
            Dialog { class: "modal backdrop:backdrop-blur",
                open: open(),
                on_close: move |_| {
                    *open.write() = false;
                    *maybe_skill.write() = None;
                },
                DialogPanel { class: "modal-box max-w-full h-full p-0",
                    div { class: "mt-12",
                        SkillView { skill }
                    }
                }
            }
        }
    } else {
        None
    }
}

#[component]
fn SkillLink(skill: Signal<Skill>, on_click: EventHandler<Signal<Skill>>) -> Element {
    #[component]
    fn SkillLinkInnerIcon(
        skill: Signal<Skill>,
        class: &'static str,
        size: i32,
        on_click: EventHandler<Signal<Skill>>,
    ) -> Element {
        rsx! {
            button { class: "hover:bg-primary border-primary border-solid border-2 rounded-md p-1 {class}",
                onclick: move |_| on_click.call(skill.clone()),
                span { class: "relative",
                    SpriteIcon { class: "rounded-md",
                        sprite: Signal::new(skill().modes[0].icon.clone()),
                        size,
                    }
                    span { class: "absolute right-0 bg-black/50 text-white text-xs px-1 text-right",
                        "{skill().name}"
                    }
                }
            }
        }
    }

    let on_click2 = on_click.clone();
    rsx! {
        SkillLinkInnerIcon { skill, class: "inline-block md:hidden", size: 56, on_click: move |e| on_click.call(e) }
        SkillLinkInnerIcon { skill, class: "hidden md:inline-block", size: 96, on_click: move |e| on_click2.call(e) }
    }
}
