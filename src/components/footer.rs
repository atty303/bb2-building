use dioxus::prelude::*;

#[component]
pub fn Footer(cx: Scope) -> Element {
    render! {
        footer {
            class: "footer p-4 mt-8 bg-neutral text-neutral-content",
            div {
                p {
                    "2024 Created by "
                    a {
                        class: "link",
                        href: "https://twitter.com/atty303",
                        "atty303"
                    }

                    ". This site is not affiliated with Nussygame."
                }
            }
        }
    }
}