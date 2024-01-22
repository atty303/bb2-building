use crate::components::Icon;
use crate::hooks::{use_modal, ModalDialogProps};
use crate::pages::skill::SkillList;
use dioxus::prelude::*;
use dioxus_signals::Signal;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlannerState {}

impl FromStr for PlannerState {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for PlannerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(s) => f.write_str(&s),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

#[component]
pub fn PlannerPage(cx: Scope, state: PlannerState) -> Element {
    let skill_modal = use_modal(cx, "max-w-full h-full".to_string());

    render! {
        div { class: "flex flex-col gap-4",
            for i in 0..5 {
                PlannerSlot {
                    index: i,
                    on_click: move |_| {
                        skill_modal.show_modal(Signal::new(()), move |e| {
                            tracing::debug!("Skill modal result: {:?}", e);
                        });
                    }
                }
            }
        }

        {skill_modal.component(cx, SkillModal)}
    }
}

pub fn SkillModal<'a>(cx: Scope<'a, ModalDialogProps<'a, (), i32>>) -> Element {
    render! {
        SkillList {
            query: Signal::new("".to_string()),
            on_search: move |q: String| {
            },
        }
    }
}

#[component]
pub fn PlannerSlot<'a>(cx: Scope<'a>, index: i32, on_click: EventHandler<'a, ()>) -> Element {
    render! {
        div { class: "bg-base-300 rounded p-4",
            onclick: move |_| on_click.call(()),
            span { class: "hover:bg-primary border-primary border-solid border-2 rounded-md p-1 inline-block",
                onclick: move |_| {

                },
                span { class: "relative",
                    span { class: "inline-block align-middle overflow-hidden",
                        width: "48px",
                        height: "48px",
                        line_height: "48px",
                        Icon {
                            class: "text-primary",
                            svg: r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-12 h-12"><path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" /></svg>"#,
                        }
                    }
                }
            }

        }
    }
}
