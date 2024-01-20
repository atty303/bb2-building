use dioxus::prelude::*;

#[component]
pub fn Icon<'a>(cx: Scope, class: Option<&'a str>, svg: &'a str) -> Element<'a> {
    let class = class.unwrap_or("");
    render! {
        span { class: "{class}",
            dangerous_inner_html: "{svg}"
        }
    }
}
