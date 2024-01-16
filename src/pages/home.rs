use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use crate::pages::Route;

#[component]
pub fn Home(cx: Scope) -> Element {
    let nav = use_navigator(cx);

    nav.replace(Route::SkillListPage { tab: "".to_string() });

    render! {
        h1 { "BB2B" }
    }
}
