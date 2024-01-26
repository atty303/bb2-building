#![allow(non_snake_case)]

use data::LANGUAGES;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use home::Home;
use planner::PlannerPage;
use rune::{RuneDebugPage, RuneListPage, RuneListState, RunePage};
use skill::{SkillDebugPage, SkillListPage, SkillListState, SkillPage};

use crate::global::{LANGUAGE, THEME};
use crate::pages::planner::PlannerState;
use crate::ui::Icon;

mod home;
mod planner;
mod rune;
mod skill;

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},

    #[route("/planner?:state")]
    PlannerPage { state: PlannerState },

    #[route("/skill?:state")]
    SkillListPage { state: SkillListState },
    #[route("/skill/_debug")]
    SkillDebugPage {},
    #[route("/skill/:skill_id")]
    SkillPage { skill_id: String },

    #[route("/rune?:state")]
    RuneListPage { state: RuneListState },
    #[route("/rune/_debug")]
    RuneDebugPage {},
    #[route("/rune/:rune_id")]
    RunePage { rune_id: String },

    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}

#[component]
fn MainLayout() -> Element {
    rsx! {
        div { class: "drawer",
            input { class: "drawer-toggle",
                r#type: "checkbox",
                id: "main-drawer-toggle",
            }
            div { class: "drawer-content",
                header { class: "sticky top-0 z-50 shadow-lg",

                    div {
                        class: "navbar bg-neutral text-neutral-content",
                        div {
                            class: "navbar-start gap-2",
                            div { class: "flex-none lg:hidden",
                                label { class: "btn btn-square btn-ghost",
                                    r#for: "main-drawer-toggle",
                                    aria_label: "open sidebar",
                                    Icon { class: "w-6 h-6",
                                        svg: r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25H12" /></svg>"#,
                                    }
                                }
                            }
                            Link { class: "link link-hover text-lg px-2 font-['Pixelify_Sans']",
                                to: Route::Home {},
                                "BB2B"
                            }
                            div { class: "hidden lg:block",
                                ul { class: "menu menu-horizontal",
                                    NavMenu {}
                                }
                            }
                        }
                        div { class: "navbar-end",
                            ThemeSelect {}
                            LanguageSelect {}
                            a { class: "btn btn-ghost btn-sm rounded-btn",
                                href: "https://github.com/atty303/bb2-building",
                                dangerous_inner_html: r#"<svg class="inline-block w-5 h-5 fill-current" width="96" height="96" viewBox="0 0 96 96" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" clip-rule="evenodd" d="M48.854 0C21.839 0 0 22 0 49.217c0 21.756 13.993 40.172 33.405 46.69 2.427.49 3.316-1.059 3.316-2.362 0-1.141-.08-5.052-.08-9.127-13.59 2.934-16.42-5.867-16.42-5.867-2.184-5.704-5.42-7.17-5.42-7.17-4.448-3.015.324-3.015.324-3.015 4.934.326 7.523 5.052 7.523 5.052 4.367 7.496 11.404 5.378 14.235 4.074.404-3.178 1.699-5.378 3.074-6.6-10.839-1.141-22.243-5.378-22.243-24.283 0-5.378 1.94-9.778 5.014-13.2-.485-1.222-2.184-6.275.486-13.038 0 0 4.125-1.304 13.426 5.052a46.97 46.97 0 0 1 12.214-1.63c4.125 0 8.33.571 12.213 1.63 9.302-6.356 13.427-5.052 13.427-5.052 2.67 6.763.97 11.816.485 13.038 3.155 3.422 5.015 7.822 5.015 13.2 0 18.905-11.404 23.06-22.324 24.283 1.78 1.548 3.316 4.481 3.316 9.126 0 6.6-.08 11.897-.08 13.526 0 1.304.89 2.853 3.316 2.364 19.412-6.52 33.405-24.935 33.405-46.691C97.707 22 75.788 0 48.854 0z" /></svg>"#,
                            }
                        }
                    }
                }
                main { class: "container my-4 mx-auto",
                    Outlet::<Route> {}
                }
                footer { class: "footer p-8 bg-neutral text-neutral-content",
                    div {
                        p {
                            "2024 Created by "
                            a { class: "link",
                                href: "https://twitter.com/atty303",
                                "atty303"
                            }
                            ". This site is not affiliated with Nussygame."
                        }
                    }
                }
            }
            div { class: "drawer-side",
                label { class: "drawer-overlay",
                    r#for: "main-drawer-toggle",
                    aria_label: "close sidebar",
                }
                ul { class: "menu p-4 w-40 min-h-full bg-neutral text-neutral-content mt-16",
                    NavMenu {}
                }
            }
        }
    }
}

#[component]
fn NavMenu() -> Element {
    rsx! {
        li {
            Link {
                to: Route::PlannerPage { state: PlannerState::default() },
                "Planner"
            }
        }
        li {
            Link {
                to: Route::SkillListPage { state: SkillListState::default() },
                "Skill"
            }
        }
        li {
            Link {
                to: Route::RuneListPage { state: RuneListState::default() },
                "Rune"
            }
        }
    }
}

#[component]
fn LanguageSelect() -> Element {
    rsx! {
        div { class: "dropdown dropdown-end",
            tabindex: 0,
            div { class: "btn btn-ghost btn-sm rounded-btn",
                role: "button",
                Icon {
                    svg: r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="m10.5 21 5.25-11.25L21 21m-9-3h7.5M3 5.621a48.474 48.474 0 0 1 6-.371m0 0c1.12 0 2.233.038 3.334.114M9 5.25V3m3.334 2.364C11.176 10.658 7.69 15.08 3 17.502m9.334-12.138c.896.061 1.785.147 2.666.257m-4.589 8.495a18.023 18.023 0 0 1-3.827-5.802" /></svg>"#,
                }
            }
            div { class: "p-2 shadow menu dropdown-content bg-base-100 text-base-content rounded-box z-10 max-h-fit overflow-y-auto w-48",
                tabindex: 0,
                div { class: "grid grid-cols-1 gap-2 p-4",
                    for t in LANGUAGES.iter() {
                        button { class: "btn btn-ghost btn-sm justify-start px-4 py-2",
                            onclick: move |_| *LANGUAGE.write() = Some(t.to_string()),
                            "{t}"
                        }
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
fn ThemeSelect() -> Element {
    rsx! {
        div { class: "dropdown dropdown-end",
            div { class: "btn btn-ghost btn-sm rounded-btn",
                tabindex: 0,
                role: "button",
                Icon {
                    svg: r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M4.098 19.902a3.75 3.75 0 0 0 5.304 0l6.401-6.402M6.75 21A3.75 3.75 0 0 1 3 17.25V4.125C3 3.504 3.504 3 4.125 3h5.25c.621 0 1.125.504 1.125 1.125v4.072M6.75 21a3.75 3.75 0 0 0 3.75-3.75V8.197M6.75 21h13.125c.621 0 1.125-.504 1.125-1.125v-5.25c0-.621-.504-1.125-1.125-1.125h-4.072M10.5 8.197l2.88-2.88c.438-.439 1.15-.439 1.59 0l3.712 3.713c.44.44.44 1.152 0 1.59l-2.879 2.88M6.75 17.25h.008v.008H6.75v-.008Z" /></svg>"#,
                }
            }
            div { class: "p-2 shadow menu dropdown-content bg-base-100 text-base-content rounded-box z-10 max-h-96 overflow-y-auto w-48",
                tabindex: 0,
                div { class: "grid grid-cols-1 gap-2 p-4",
                    for t in THEMES.iter() {
                        button {
                            class: "btn btn-ghost btn-sm justify-start px-4 py-2",
                            onclick: move |_| *THEME.write() = t.to_string(),
                            "{t}"
                        }
                    }
                }
            }
        }
    }
}
