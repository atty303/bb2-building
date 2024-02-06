use dioxus::prelude::*;
use markdown_it::MarkdownIt;

#[component]
pub fn BuildEditPage() -> Element {
    let mut doc = use_signal(|| "".to_string());

    let rendered = use_memo(move || {
        let node = MarkdownIt::new().parse(&doc());
    });

    rsx! {
        textarea {
            id: "build-edit",
            rows: 10,
            cols: 50,
            oninput: move |e| {
                *doc.write() = e.data.value();
            },
            "{doc}"
        }
    }
}
