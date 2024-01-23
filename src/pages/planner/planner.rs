use crate::components::{Icon, SkillView, SpriteIcon};
use crate::hooks::{use_modal, ModalDialogProps};
use crate::pages::skill::SkillList;
use crate::pages::Route;
use data::skill::{Skill, SkillHash};
use data::RuneHash;
use dioxus::prelude::*;
use dioxus_signals::{use_signal, Signal};
use fermi::use_read;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlannerState {
    build: BuildState,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
struct BuildState {
    slots: [SlotState; 5],
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
struct SlotState {
    skill: Option<SkillHash>,
    runes: [RuneHash; 5],
}

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
    let skill_modal = use_modal(cx, "max-w-full h-full p-0".to_string());

    tracing::info!("state: {:?}", state);

    render! {
        div { class: "flex flex-col gap-4",
            for i in 0..5 {
                PlannerSlot {
                    index: i,
                    state: state.build.slots[i as usize].clone(),
                    on_click: {
                        let state = state.clone();
                        move |_| {
                            let state = state.clone();
                            skill_modal.show_modal(Signal::new(()), move |e| {
                                let mut state = state.clone();
                                state.build.slots[i as usize].skill = Some(e);
                                let router = dioxus_router::router();
                                router.replace(Route::PlannerPage {
                                    state: state,
                                });
                            });
                        }
                    }
                }
            }
        }

        {skill_modal.component(cx, SkillModal)}
    }
}

pub fn SkillModal<'a>(cx: Scope<'a, ModalDialogProps<'a, (), SkillHash>>) -> Element {
    let query = use_signal(cx, || "".to_string());
    let selected = use_signal(cx, || None::<Signal<Skill>>);
    render! {
        div { class: "sticky top-0 bg-base-300 p-2 z-10 mb-2",
            if let Some(skill) = *selected.read() {
                button { class: "btn btn-primary btn-sm",
                    onclick: move |_| {
                        cx.props.on_result.call(skill().hash);
                    },
                    "Select {skill().name}"
                }
            } else {
                button { class: "btn btn-primary btn-sm btn-disabled",
                    "Select"
                }
            }
        }
        div { class: "p-4",
            SkillList {
                query: query.clone(),
                on_search: move |q: String| {
                    query.set(q);
                },
                selected: selected,
            }
        }
    }
}

#[component]
fn PlannerSlot<'a>(
    cx: Scope<'a>,
    index: i32,
    state: SlotState,
    on_click: EventHandler<'a, ()>,
) -> Element {
    let db = use_read(cx, &crate::atoms::DATABASE);
    let maybe_skill = state
        .skill
        .and_then(|hash| db.skill.get(&hash))
        .map(|s| Signal::new(s.clone()));

    render! {
        div { class: "collapse collapse-arrow",
            tabindex: "{index}",
            div { class: "collapse-title px-2 py-0",
                span { class: "badge badge-neutral mr-2",
                    "{index + 1}"
                }
                if let Some(skill) = maybe_skill {
                    span { class: "hover:bg-primary border-primary border-solid border-2 rounded-md p-1 inline-block",
                        onclick: move |_| on_click.call(()),
                        span { class: "relative",
                            SpriteIcon { class: "rounded-md",
                                sprite: Signal::new(skill.read().modes[0].icon.clone()),
                                size: 48,
                            }
                        }
                    }
                    span { class: "ml-2",
                        "{skill().name}"
                    }
                } else {
                    span { class: "hover:bg-primary border-primary border-solid border-2 rounded-md p-1 inline-block",
                        onclick: move |_| on_click.call(()),
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
            div { class: "collapse-content",
                if let Some(skill) = maybe_skill {
                    SkillView { skill: skill }
                }
            }
        }
    }
}
