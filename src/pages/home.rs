use dioxus::prelude::*;
use dioxus::router::router;

use crate::pages::skill::SkillListState;
use crate::pages::Route;

#[component]
pub fn Home() -> Element {
    router().replace(Route::SkillListPage {
        state: SkillListState::default(),
    });

    rsx! { h1 { "BB2B" } }
}
