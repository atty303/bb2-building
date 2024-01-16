use dioxus::prelude::*;

#[component]
pub fn Icon<'a>(cx: Scope, class: &'a str, svg: &'a str) -> Element<'a> {
    render! {
        span { class: "{class}",
            dangerous_inner_html: "{svg}"
        }
    }
}
