use std::fmt::Display;
use std::str::FromStr;

use dioxus::prelude::*;
use dioxus::router::router;
use serde::{Deserialize, Serialize};

use crate::components::SkillView;
use data::skill::Skill;
use data::SkillHash;

use crate::global::DATABASE;
use crate::hooks::use_search_skill;
use crate::pages::Route;
use crate::ui::{Dialog, SpriteIcon};
use crate::Language;

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
pub fn SkillListPage(language: Language, state: SkillListState) -> Element {
    rsx! {
        div { class: "text-sm breadcrumbs",
            ul {
                li { "Home" }
                li { "Skill" }
            }
        }

        SkillList {
            language: language.clone(),
            query: state.query,
            on_search: move |q: String| {
                router()
                    .replace(Route::SkillListPage {
                        language: language.clone(),
                        state: SkillListState {
                            query: Signal::new(q.clone()),
                        },
                    });
            }
        }
    }
}

#[component]
pub fn SkillList(
    language: Language,
    query: Signal<String>,
    on_search: EventHandler<String>,
    on_select: Option<EventHandler<SkillHash>>,
) -> Element {
    let mut detail_open = use_signal(|| false);
    let mut detail_skill = use_signal(|| None);

    let mut search = use_search_skill();
    if *search.query.peek() != *query.peek() {
        *search.query.write() = query.peek().clone();
    }

    rsx! {
        div {
            div { class: "flex flex-row items-center gap-4",
                div { class: "relative flex-grow",
                    input {
                        class: "input input-bordered input-primary w-full",
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
                }
                div { class: "badge badge-accent badge-lg gap-1 text-xs",
                    span { class: "font-bold", "{search.results.read().len()}" }
                    span { "of" }
                    span { class: "font-bold", "{DATABASE().skill.iter().count()}" }
                }
            }

            div { class: "flex flex-wrap gap-2 mt-4",
                for skill in search.results.read().iter() {
                    SkillLink {
                        skill: *skill,
                        on_click: move |skill: Signal<Skill>| {
                            *detail_open.write() = true;
                            *detail_skill.write() = Some(skill.clone());
                        }
                    }
                }
            }
        }

        DetailDialog {
            language,
            open: detail_open,
            maybe_skill: detail_skill,
            selectable: on_select.is_some(),
            on_select: move |e| on_select.clone().map_or((), |h| h.call(e))
        }
    }
}

#[component]
pub fn DetailDialog(
    language: Language,
    open: Signal<bool>,
    maybe_skill: Signal<Option<Signal<Skill>>>,
    selectable: bool,
    on_select: EventHandler<SkillHash>,
) -> Element {
    if let Some(skill) = maybe_skill() {
        rsx! {
            Dialog {
                open,
                on_close: move |_| {
                    *open.write() = false;
                    *maybe_skill.write() = None;
                },
                if selectable {
                    div { class: "sticky top-0 h-0 p-2",
                        button {
                            class: "btn btn-primary btn-sm",
                            onclick: move |_| {
                                on_select.call(skill().hash.clone());
                                *open.write() = false;
                                *maybe_skill.write() = None;
                            },
                            "Select"
                        }
                    }
                }
                div { class: "mt-12", SkillView { language, skill } }
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
            button {
                class: "hover:bg-primary border-primary border-solid border-2 rounded-md p-1 {class}",
                onclick: move |_| on_click.call(skill.clone()),
                span { class: "relative",
                    SpriteIcon {
                        class: "rounded-md",
                        sprite: Signal::new(skill().modes[0].icon.clone()),
                        size
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
        SkillLinkInnerIcon {
            skill,
            class: "inline-block md:hidden",
            size: 56,
            on_click: move |e| on_click.call(e)
        }
        SkillLinkInnerIcon {
            skill,
            class: "hidden md:inline-block",
            size: 96,
            on_click: move |e| on_click2.call(e)
        }
    }
}
