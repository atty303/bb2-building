use crate::ui::{Dialog, DialogPanel, DialogRenderArgs, RenderFn};
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        div { class: "w-full h-full",
            button { class: "btn btn-primary",
                onclick: move |_| {
                    tracing::info!("clicked");
                },
                "Click me!"
            }
        }
        DaisyDialog {
            div { class: "font-bold",
                "Hello world!"
            }
        }
    }
}

#[component]
fn DaisyDialog(children: Element) -> Element {
    let mut open = use_signal(|| true);

    rsx! {
        Dialog {
            open: open(),
            on_close: move |_| {
                open.set(false);
            },
            // render: RenderFn::new(|args: DialogRenderArgs| {
            //     rsx! {
            //         dialog {
            //             ..args.attrs,
            //             {args.children}
            //         }
            //     }
            // }),
            DialogPanel {
                // render: |attrs, children, open| {
                //     rsx! {
                //         div { class: "",
                //             ..attrs,
                //             {children}
                //         }
                //     }
                // },
                {children}
            }
        }
    }
}
