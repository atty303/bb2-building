use crate::pages::Route;
use dioxus::prelude::*;
use dioxus::router::prelude::Link;

use crate::ui::{Description, Rarity, SpriteIcon};

#[component]
pub fn SkillView(skill: Signal<data::Skill>, #[props(default = false)] debug: bool) -> Element {
    rsx! {
        div { class: "flex flex-col border-solid border border-base-300 rounded-md my-2",
            div { class: "flex flex-row items-center gap-2 bg-base-300 text-base-content p-2",
                SpriteIcon { class: "rounded-md", sprite: Signal::new(skill().modes[0].icon.clone()), size: 48 }
                span { class: "flex-grow",
                    Link { class: "text-primary hover:underline cursor-pointer",
                        to: Route::SkillPage { skill_id: skill().id.clone() },
                        "{skill().name}"
                    }
                }
                span {
                    Rarity { rarity: skill().rarity }
                }
            }
            div { class: "flex flex-row flex-wrap gap-2 p-2",
                for mode in skill().modes.iter() {
                    div { class: "flex-1 min-w-64",
                        SkillMode { mode: Signal::new(mode.clone()), debug }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SkillMode(mode: Signal<data::SkillMode>, #[props(default = false)] debug: bool) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 bg-base-200 text-base-content rounded-md p-2",
            div { class: "flex flex-row items-center gap-2",
                SpriteIcon { class: "rounded-md", sprite: Signal::new(mode().icon), size: 32 }
                div { class: "flex-grow",
                    "{mode().name}"
                }
            }
            div { class: "bg-base-100 p-2",
                Description { tokens: mode().format(), debug }
            }
        }
    }
}
