#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod home;
pub mod rune;
pub mod skill;

use crate::components::Footer;
use crate::components::NavBar;
use crate::pages::rune::RuneListQuery;
use crate::pages::skill::SkillListQuery;
use home::Home;
use rune::{RuneDebugPage, RuneListPage, RunePage};
use skill::{SkillDebugPage, SkillListPage, SkillPage};

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
pub enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},

    #[route("/skill?:query")]
    SkillListPage { query: SkillListQuery },
    #[route("/skill/_debug")]
    SkillDebugPage {},
    #[route("/skill/:skill_id")]
    SkillPage { skill_id: String },

    #[route("/rune?:query")]
    RuneListPage { query: RuneListQuery },
    #[route("/rune/_debug")]
    RuneDebugPage {},
    #[route("/rune/:rune_id")]
    RunePage { rune_id: String },

    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn MainLayout(cx: Scope) -> Element {
    render! {
        header { class: "sticky top-0 z-50 shadow-lg",
            NavBar {}
        }
        main { class: "container my-4 mx-auto",
            Outlet::<Route> {}
        }
        Footer {}
    }
}

#[component]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}
