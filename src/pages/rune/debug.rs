use dioxus::prelude::*;
use dioxus_signals::Signal;
use fermi::use_read;

use crate::atoms::DATABASE;
use crate::components::RuneView;

#[component]
pub fn RuneDebugPage(cx: Scope) -> Element {
    let db = use_read(cx, &DATABASE);

    render! {
        for rune in db.rune.iter() {
            RuneView { rune: Signal::new(rune.clone()), debug: true }
        }
    }
}
