use dioxus::prelude::*;
use dioxus::router::router;
use dioxus_signals::Signal;

use crate::components::SkillView;
use crate::global::DATABASE;
use crate::pages::planner::{PlannerState, SlotState};
use crate::pages::skill::SkillList;
use crate::pages::Route;
use crate::ui::SpriteIcon;

#[component]
pub fn PlannerPage(state: PlannerState) -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            for i in 0..5 {
                PlannerSlot {
                    index: i,
                    state: state.build.slots[i as usize].clone(),
                    on_click: {
                        let state = state.clone();
                        move |_| {
                            let state = state.clone();
                            router().replace(Route::PlannerEditSlotPage {
                                state,
                                index: i,
                            });
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PlannerEditSlotPage(state: PlannerState, index: i32) -> Element {
    let mut query = use_signal(|| String::default());

    rsx! {
        SkillList {
            query,
            on_search: move |q: String| {
                *query.write() = q;
            },
            on_select: move |hash| {
                let mut state = state.clone();
                state.build.slots[index as usize].skill = Some(hash);
                let router = dioxus_router::router();
                router.replace(Route::PlannerPage {
                    state: state,
                });
            },
        }
    }
}

#[component]
fn PlannerSlot(index: i32, state: SlotState, on_click: EventHandler<()>) -> Element {
    let maybe_skill = state
        .skill
        .and_then(|hash| DATABASE().skill.get(&hash).map(|s| Signal::new(s.clone())));

    rsx! {
        if let Some(skill) = maybe_skill {
            div { class: "collapse collapse-arrow",
                tabindex: "{index}",
                div { class: "collapse-title px-2 py-1",
                    span { class: "badge badge-neutral mr-2",
                        "{index + 1}"
                    }
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
                }
                div { class: "collapse-content",
                    SkillView { skill: skill }

                    // for i in 0..5 {
                    //     button { class: "ml-2 btn btn-primary btn-wide",
                    //         "Click to select rune"
                    //     }
                    // }
                }
            }
        } else {
            div { class: "px-2 py-1",
                span { class: "badge badge-neutral mr-2",
                    "{index + 1}"
                }
                button { class: "ml-2 btn btn-primary btn-wide",
                    onclick: move |_| on_click.call(()),
                    "Click to select skill"
                }
            }
        }
    }
}
