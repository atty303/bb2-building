use data::skill::Skill;
use dioxus::prelude::*;
use dioxus_router::prelude::{use_navigator, Link};
use fermi::use_read_rc;

use crate::atoms::DATABASE;
use crate::components::{Rarity, SkillView, SpriteIcon};
use crate::components::{Tab, TabGroup, TabList, TabPanel, TabPanels};
use crate::pages::Route;

#[component]
fn LocalTab<'a>(cx: Scope<'a>, index: usize, children: Element<'a>) -> Element {
    render! {
        Tab {
            index: *index,
            render: move |attrs, children, selected| {
                let active = if selected { "tab-active" } else { "" };
                render! {
                    button {
                        class: "tab {active}",
                        ..*attrs,
                        {children}
                    }
                }
            },
            {children}
        }
    }
}

#[component]
fn LocalTabPanel<'a>(cx: Scope<'a>, index: usize, children: Element<'a>) -> Element {
    render! {
        TabPanel {
            index: *index,
            render: move |attrs, children, selected| {
                let active = if selected { "" } else { "" };
                render! {
                    div {
                        class: "{active}",
                        ..*attrs,
                        {children}
                    }
                }
            },
            {children}
        }
    }
}

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

        TabGroup {
            render: move |children, _selected_index| {
                render! {
                    div {
                        {children}
                    }
                }
            },
            TabList {
                render: move |attrs, children, _selected_index| {
                    render! {
                        div {
                            class: "tabs tabs-bordered tab-lg text-primary mb-2",
                            ..*attrs,
                            {children}
                        }
                    }
                },
                LocalTab {
                    index: 0,
                    "ALL",
                }
                LocalTab {
                    index: 1,
                    "RARITY",
                }
            }
            TabPanels {
                render: move |attrs, children, _selected_index| {
                    render! {
                        div {
                            ..*attrs,
                            {children}
                        }
                    }
                },
                LocalTabPanel {
                    index: 0,
                    div { class: "grid grid-cols-5 w-fit gap-2",
                        for skill in db.skill.iter() {
                            SkillLink { skill: &skill }
                        }
                    }
                }
                LocalTabPanel {
                    index: 1,
                    div { class: "p-2 divide-y",
                        for rarity in rarities {
                            div { class: "py-4",
                                h2 { class: "mb-2",
                                    Rarity { rarity: rarity }
                                }
                                div { class: "flex flex-wrap gap-2",
                                    for skill in db.skill.iter().filter(|s| s.rarity == rarity) {
                                        SkillLink { skill: &skill }
                                    }
                                }
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
        span { class: "inline-block",
            title: "{skill.name}",
            Link { class: "inline-block hover:bg-primary border-primary border-solid border-2 rounded-md p-1",
                to: Route::SkillPage { skill_id: skill.id.clone() },
                SpriteIcon { class: "rounded-md", sprite: &skill.modes[0].icon, size: 96 }
            }

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

    db.skill
        .values()
        .find(|s| &s.id == skill_id)
        .map(|skill| {
            render! {
                div {
                    class: "text-sm breadcrumbs",
                    ul {
                        li { "Home" }
                        li { "Skill" }
                        li { "{skill.name}" }
                    }
                }

                SkillView { skill: &skill }
            }
        })
        .unwrap_or_else(|| {
            render! {
                div {
                    "Skill not found"
                }
            }
        })
}
