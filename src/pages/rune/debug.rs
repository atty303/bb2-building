use dioxus::prelude::*;

use crate::components::RuneView;
use crate::global::DATABASE;
use crate::Language;

#[component]
pub fn RuneDebugPage(language: Language) -> Element {
    rsx! {
        for rune in DATABASE().rune.iter() {
            RuneView { language: language.clone(), rune: Signal::new(rune.clone()), debug: true }
        }
    }
}
