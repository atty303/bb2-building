use dioxus::html::geometry::euclid::Rect;
use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use dioxus_signals::{use_selector, use_signal, Signal};
use fermi::use_read;

use data::token::{Token, Tokens};

use crate::atoms::DATABASE;
use crate::components::{Rarity, SpriteIcon};
use crate::pages::Route;

#[component]
pub fn SkillView(
    cx: Scope,
    skill: Signal<data::skill::Skill>,
    #[props(default = false)] debug: bool,
) -> Element {
    render! {
        div {
            class: "flex flex-col border-solid border border-base-300 rounded-md my-2",
            div {
                class: "flex flex-row items-center gap-2 bg-base-300 text-base-content p-2",
                SpriteIcon { class: "rounded-md", sprite: Signal::new(skill.read().modes[0].icon.clone()), size: 48 }
                span {
                    class: "flex-grow",
                    Link {
                        class: "text-primary hover:underline cursor-pointer",
                        to: Route::SkillPage { skill_id: skill.read().id.clone() },
                        "{skill.read().name}"
                    }
                }
                span {
                    Rarity { rarity: skill.read().rarity }
                }
            }
            div { class: "flex flex-row flex-wrap gap-2 p-2",
                for mode in skill.read().modes.iter() {
                    div { class: "flex-1 min-w-64",
                        SkillMode { mode: Signal::new(mode.clone()), debug: *debug }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SkillMode(
    cx: Scope,
    mode: Signal<data::skill::SkillMode>,
    #[props(default = false)] debug: bool,
) -> Element {
    render! {
        div { class: "flex flex-col gap-2 bg-base-200 text-base-content rounded-md p-2",
            div { class: "flex flex-row items-center gap-2",
                SpriteIcon { class: "rounded-md", sprite: Signal::new(mode.read().icon.clone()), size: 32 }
                div { class: "flex-grow",
                    "{mode.read().name}"
                }
            }
            div { class: "bg-base-100 p-2",
                Description { tokens: mode.read().format(), debug: *debug }
            }
        }
    }
}
