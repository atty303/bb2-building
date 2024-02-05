use std::fmt::Display;

use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let s = use_signal(|| 0);
    rsx! { "{s}" }
}
