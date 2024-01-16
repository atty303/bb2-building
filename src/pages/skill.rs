use dioxus::prelude::*;
use dioxus_router::prelude::{Link, use_navigator};
use fermi::use_read_rc;
use data::skill::Skill;

use crate::atoms::DATABASE;
use crate::components::rarity::Rarity;
use crate::components::skill_view::SkillView;
use crate::components::sprite::SpriteIcon;
use crate::pages::Route;

#[component]
pub fn SkillListPage(cx: Scope, tab: String) -> Element {
    let nav = use_navigator(cx);
    let db = use_read_rc(cx, &DATABASE);
    let rarities = db.skill.rarity_range();

    render! {
        div {
            class: "text-sm breadcrumbs",
            ul {
                li { "Home" }
                li { "Skill" }
            }
        }

        Link { class: "btn btn-primary btn-sm my-2",
            to: Route::SkillSearchPage {},
            "Search"
        }

        div { class: "tabs tabs-bordered",
            role: "tablist",

            input { class: "tab text-primary",
                r#type: "radio",
                name: "tabs",
                role: "tab",
                aria_label: "All",
                checked: tab != "rarity",
                onclick: move |_| {
                    nav.replace(Route::SkillListPage { tab: "all".to_string() });
                },
            }
            div { class: "tab-content p-2",
                role: "tabpanel",
                div { class: "grid grid-cols-5 w-fit gap-2",
                    for skill in db.skill.iter().filter(|s| s.in_dictionary) {
                        SkillLink { skill: &skill }
                    }
                }
            }

            input { class: "tab text-primary",
                r#type: "radio",
                name: "tabs",
                role: "tab",
                aria_label: "Rarity",
                checked: tab == "rarity",
                onclick: move |_| {
                    nav.replace(Route::SkillListPage { tab: "rarity".to_string() });
                },
            }
            div { class: "tab-content p-2 divide-y",
                role: "tabpanel",
                for rarity in rarities {
                    div { class: "py-4",
                        h2 { class: "mb-2",
                            Rarity { rarity: rarity }
                        }
                        div { class: "flex flex-wrap gap-2",
                            for skill in db.skill.iter().filter(|s| s.in_dictionary && s.rarity == rarity) {
                                SkillLink { skill: &skill }
                            }
                        }
                    }
                }
            }
        }

    }
}

#[component]
fn SkillLink<'a>(cx: Scope, skill: &'a Skill) -> Element<'a> {
    render! {
        Link { class: "inline-block hover:bg-primary border-primary border-solid border-2 rounded-md p-1",
            to: Route::SkillPage { skill_id: skill.id.clone() },
            SpriteIcon { class: "rounded-md", sprite: &skill.modes[0].icon, size: 96 }
        }
    }
}

#[component]
pub fn SkillSearchPage(cx: Scope) -> Element {
    let db = use_read_rc(cx, &DATABASE);

    render! {
        for skill in db.skill.iter().filter(|s| s.in_dictionary) {
            SkillView { skill: &skill }
        }
    }
}

#[component]
pub fn SkillPage(cx: Scope, skill_id: String) -> Element {
    let db = use_read_rc(cx, &DATABASE);

    db.skill.values().find(|s| &s.id == skill_id).map(|skill| {
        render! {
            div {
                class: "text-sm breadcrumbs",
                ul {
                    li { "Home" }
                    li { "Skill" }
                    li { skill.name(db.term()) }
                }
            }

            SkillView { skill: &skill }
        }
    }).unwrap_or_else(|| {
        render! {
            div {
                "Skill not found"
            }
        }
    })
}
