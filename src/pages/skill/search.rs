use crate::components::SkillView;
use crate::hooks::use_search_skill;
use crate::pages::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

#[component]
pub fn SkillSearchPage<'a>(cx: Scope<'a>) -> Element {
    let nav = use_navigator(cx);
    nav.replace(Route::SkillListPage {});
    None
}
