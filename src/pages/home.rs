use crate::pages::Route;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;

#[component]
pub fn Home() -> Element {
    // let nav = use_navigator(cx);

    // nav.replace(Route::SkillListPage {
    //     query: SkillListQuery::default(),
    // });

    rsx! {
        h1 { "BB2B" }
    }
}
