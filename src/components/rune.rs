use crate::Language;
use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::pages::Route;
use crate::ui::{Description, Rarity, SpriteIcon};

#[component]
pub fn RuneView(
    language: Language,
    rune: Signal<data::Rune>,
    #[props(default = false)] debug: bool,
) -> Element {
    rsx! {
        div { class: "flex flex-col border-solid border border-base-300 rounded-md my-2",
            div { class: "flex flex-row items-center gap-2 bg-base-300 text-base-content p-2",
                SpriteIcon { class: "rounded-md", sprite: Signal::new(rune().icon), size: 48 }
                span { class: "flex-grow",
                    Link {
                        class: "text-primary hover:underline cursor-pointer",
                        to: Route::RunePage {
                            language,
                            rune_id: rune().id,
                        },
                        "{rune().name}"
                    }
                }
                span { Rarity { rarity: rune().rarity } }
            }
            div { class: "flex flex-row flex-wrap gap-2 p-2", Description { tokens: rune().format(), debug } }
        }
    }
}
