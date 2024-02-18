use crate::components::RuneView;
use dioxus::prelude::*;

use crate::global::DATABASE;
use crate::Language;

#[component]
pub fn RunePage(language: Language, rune_id: String) -> Element {
    DATABASE()
        .rune
        .iter()
        .find(|s| s.id == rune_id)
        .map(|rune| {
            rsx! {
                div { class: "text-sm breadcrumbs",
                    ul {
                        li { "Home" }
                        li { "Rune" }
                        li { "{rune.name}" }
                    }
                }

                RuneView { language, rune: Signal::new(rune.clone()) }
            }
        })
        .unwrap_or_else(|| {
            rsx! { div { "Rune not found" } }
        })
}
