#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod home;
mod skill;

use crate::components::Footer;
use crate::components::NavBar;
use home::Home;
use skill::{SkillListPage, SkillPage, SkillSearchPage};

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
pub enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},
    #[route("/skill")]
    SkillListPage {},
    #[route("/skill/search")]
    SkillSearchPage {},
    #[route("/skill/:skill_id")]
    SkillPage { skill_id: String },
    //#[end_layout]
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
