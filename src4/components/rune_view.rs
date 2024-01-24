use crate::components::{Description, Rarity, SpriteIcon};
use crate::pages::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use dioxus_signals::Signal;

#[component]
pub fn RuneView(
    cx: Scope,
    rune: Signal<data::Rune>,
    #[props(default = false)] debug: bool,
) -> Element {
    render! {
        div {
            class: "flex flex-col border-solid border border-base-300 rounded-md my-2",
            div {
                class: "flex flex-row items-center gap-2 bg-base-300 text-base-content p-2",
                SpriteIcon { class: "rounded-md", sprite: Signal::new(rune.read().icon.clone()), size: 48 }
                span {
                    class: "flex-grow",
                    Link {
                        class: "text-primary hover:underline cursor-pointer",
                        to: Route::RunePage { rune_id: rune.read().id.clone() },
                        "{rune.read().name}"
                    }
                }
                span {
                    Rarity { rarity: rune.read().rarity }
                }
            }
            div { class: "flex flex-row flex-wrap gap-2 p-2",
                Description { tokens: rune.read().format(), debug: *debug }
            }
        }
    }
}
