use dioxus::prelude::*;
use dioxus_signals::Signal;
use fermi::use_read;

use crate::atoms::DATABASE;
use crate::components::RuneView;

#[component]
pub fn RunePage(cx: Scope, rune_id: String) -> Element {
    let db = use_read(cx, &DATABASE);

    db.rune
        .iter()
        .find(|s| &s.id == rune_id)
        .map(|rune| {
            render! {
                div { class: "text-sm breadcrumbs",
                    ul {
                        li { "Home" }
                        li { "Rune" }
                        li { "{rune.name}" }
                    }
                }

                RuneView { rune: Signal::new(rune.clone()) }
            }
        })
        .unwrap_or_else(|| {
            render! {
                div {
                    "Rune not found"
                }
            }
        })
}
