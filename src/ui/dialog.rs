use dioxus::prelude::*;

use crate::ui::Icon;

#[component]
pub fn Dialog(
    open: Signal<bool>,
    on_close: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    let on_close1 = on_close.clone();
    rsx! {
        dioxus_headlessui::dialog::Dialog { class: "modal backdrop:backdrop-blur",
            open: open(),
            on_close: move |_| {
                *open.write() = false;
                on_close.as_ref().map(|h| h.call(()));
            },
            dioxus_headlessui::dialog::DialogPanel { class: "modal-box max-w-full h-full p-0",
                div { class: "sticky block top-0 right-0 left-0 w-full h-0 z-50",
                    button { class: "btn btn-sm btn-circle btn-neutral absolute right-2 top-2",
                        tabindex: -1,
                        onclick: move |_| {
                            *open.write() = false;
                            on_close1.as_ref().map(|h| h.call(()));
                        },
                        Icon {
                            svg: r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" /></svg>"#,
                        }
                    }
                }
                div { class: "mt-12",
                    {children}
                }
            }
        }
    }
}
