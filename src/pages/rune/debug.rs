use dioxus::prelude::*;

use crate::components::RuneView;
use crate::global::DATABASE;

#[component]
pub fn RuneDebugPage() -> Element {
    rsx! {
        for rune in DATABASE().rune.iter() {
            RuneView { rune: Signal::new(rune.clone()), debug: true }
        }
    }
}
