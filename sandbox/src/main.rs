use std::fmt::{format, Display};

use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let b = use_resource(move || async move {
        reqwest::get("https://httpbin.org/ip")
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    });

    match b.value().as_ref() {
        None => rsx! { "Loading..." },
        Some(v) => {
            rsx! { "{v}" }
        }
    }
}
