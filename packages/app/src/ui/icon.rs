use dioxus::prelude::*;

#[component]
pub fn Icon(class: Option<&'static str>, svg: &'static str) -> Element {
    let class = class.unwrap_or("");
    rsx! {
        span { class,
            dangerous_inner_html: "{svg}"
        }
    }
}
