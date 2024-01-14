#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod skill;
mod home;

use home::Home;
use skill::SkillListPage;
use crate::components::footer::Footer;
use crate::components::nav_bar::NavBar;

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
pub enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},
    #[route("/skill")]
    SkillListPage {},
    //#[end_layout]
}

#[component]
fn MainLayout(cx: Scope) -> Element {
    render! {
        div {
            header {
                NavBar {}
            }
            main {
                class: "container my-4 mx-auto",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}