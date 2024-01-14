#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::hi_outline_icons;

use data::LANGUAGES;
use crate::components::app::Language;

use crate::hooks::persistent::use_persistent;

#[component]
pub fn NavBar(cx: Scope) -> Element {
    let base_uri = gloo_utils::document()
        .base_uri()
        .unwrap_or_default()
        .unwrap();
    render! {
        div {
            class: "navbar mb-2 shadow-lg bg-neutral text-neutral-content sticky top-0 z-10 shadow-lg",
            div {
                class: "flex-1 px-2 mx-2",
                span {
                    class: "text-lg font-bold",
                    a {
                        class: "link link-hover",
                        href: "{base_uri}",
                        "BB2B"
                    }
                }
            }
            div {
                class: "flex-none hidden px-2 mx-2 lg:flex",
                div {
                    class: "flex items-center space-x-4",

                    ThemeSelect {}
                    LanguageSelect {}

                    a {
                        class: "btn btn-ghost btn-sm rounded-btn",
                        href: "https://github.com/atty303/bb2-building",
                        dangerous_inner_html: r#"<svg class="inline-block w-5 h-5 fill-current" width="96" height="96" viewBox="0 0 96 96" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" clip-rule="evenodd" d="M48.854 0C21.839 0 0 22 0 49.217c0 21.756 13.993 40.172 33.405 46.69 2.427.49 3.316-1.059 3.316-2.362 0-1.141-.08-5.052-.08-9.127-13.59 2.934-16.42-5.867-16.42-5.867-2.184-5.704-5.42-7.17-5.42-7.17-4.448-3.015.324-3.015.324-3.015 4.934.326 7.523 5.052 7.523 5.052 4.367 7.496 11.404 5.378 14.235 4.074.404-3.178 1.699-5.378 3.074-6.6-10.839-1.141-22.243-5.378-22.243-24.283 0-5.378 1.94-9.778 5.014-13.2-.485-1.222-2.184-6.275.486-13.038 0 0 4.125-1.304 13.426 5.052a46.97 46.97 0 0 1 12.214-1.63c4.125 0 8.33.571 12.213 1.63 9.302-6.356 13.427-5.052 13.427-5.052 2.67 6.763.97 11.816.485 13.038 3.155 3.422 5.015 7.822 5.015 13.2 0 18.905-11.404 23.06-22.324 24.283 1.78 1.548 3.316 4.481 3.316 9.126 0 6.6-.08 11.897-.08 13.526 0 1.304.89 2.853 3.316 2.364 19.412-6.52 33.405-24.935 33.405-46.691C97.707 22 75.788 0 48.854 0z" /></svg>"#,
                    }
                }
            }
        }
    }
}

const THEMES: [&str; 29] = [
    "light",
    "dark",
    "cupcake",
    "bumblebee",
    "emerald",
    "corporate",
    "synthwave",
    "retro",
    "cyberpunk",
    "valentine",
    "halloween",
    "garden",
    "forest",
    "aqua",
    "lofi",
    "pastel",
    "fantasy",
    "wireframe",
    "black",
    "luxury",
    "dracula",
    "cmyk",
    "autumn",
    "business",
    "acid",
    "lemonade",
    "night",
    "coffee",
    "winter",
];

#[component]
fn ThemeSelect(cx: Scope) -> Element {
    let theme = use_persistent(cx, "theme", || "dark".to_string());
    use_effect(cx, &theme.get(), move |theme| async move {
        let root = gloo_utils::document_element();
        root.set_attribute("data-theme", theme.as_str()).unwrap();
    });

    render! {
        div {
            class: "dropdown dropdown-end",
            div {
                class: "btn btn-ghost btn-sm rounded-btn",
                tabindex: 0,
                role: "button",
                Icon {
                    class: "text-primary",
                    icon: hi_outline_icons::HiColorSwatch,
                    width: 28,
                    height: 28,
                }
                "Theme"
            }
            div {
                class: "p-2 shadow menu dropdown-content bg-base-100 text-base-content rounded-box z-10 max-h-96 overflow-y-auto w-48",
                tabindex: 0,
                div {
                    class: "grid grid-cols-1 gap-2 p-4",
                    for t in THEMES.iter() {
                        button {
                            class: "btn btn-ghost btn-sm justify-start px-4 py-2",
                            onclick: move |_| theme.set(t.to_string()),
                            "{t}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LanguageSelect(cx: Scope) -> Element {
    let language_state = use_shared_state::<Language>(cx).unwrap();
    let language_persistent = use_persistent(cx, "language", || "en".to_string());
    use_effect(cx, &language_persistent.get(), move |language| {
        to_owned![language_state];
        async move {
            if let Some(lang) = LANGUAGES.iter().find(|lang| *lang == &language.as_str()) {
                *language_state.write() = Language { code: lang };
            }
        }
    });

    render! {
        div {
            class: "dropdown dropdown-end",
            tabindex: 0,
            div {
                class: "btn btn-ghost btn-sm rounded-btn",
                role: "button",
                Icon {
                    icon: hi_outline_icons::HiGlobe,
                    width: 28,
                    height: 28,
                }
                "Language"
            }
            div {
                class: "p-2 shadow menu dropdown-content bg-base-100 text-base-content rounded-box z-10 max-h-fit overflow-y-auto w-48",
                tabindex: 0,
                div {
                    class: "grid grid-cols-1 gap-2 p-4",
                    for t in LANGUAGES.iter() {
                        button {
                            class: "btn btn-ghost btn-sm justify-start px-4 py-2",
                            onclick: move |_| language_persistent.set(t.to_string()),
                            "{t}"
                        }
                    }
                }
            }
        }
    }
}