use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        div { class: "bg-secondary/50 font-bold",
            "Hello world!3"
        }
    }
}
