use dioxus::prelude::*;
use dioxus::router::router;
use dioxus_signals::Signal;

use crate::components::SkillView;
use crate::global::DATABASE;
use crate::pages::planner::{PlannerState, SlotState};
use crate::pages::skill::SkillList;
use crate::pages::Route;
use crate::ui::{Icon, SpriteIcon};

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
    let query = use_signal(|| String::default());

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
                                Icon { class: "text-primary",
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
