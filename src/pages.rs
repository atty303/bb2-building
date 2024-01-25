#![allow(non_snake_case)]

mod home;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use home::Home;

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},

    // #[route("/planner?:state")]
    // PlannerPage { state: PlannerState },
    //
    // #[route("/skill?:query")]
    // SkillListPage { query: SkillListQuery },
    // #[route("/skill/_debug")]
    // SkillDebugPage {},
    // #[route("/skill/:skill_id")]
    // SkillPage { skill_id: String },
    //
    // #[route("/rune?:query")]
    // RuneListPage { query: RuneListQuery },
    // #[route("/rune/_debug")]
    // RuneDebugPage {},
    // #[route("/rune/:rune_id")]
    // RunePage { rune_id: String },
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
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
                    // NavBar {}
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
                // SideMenu {}
            }
        }
    }
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
