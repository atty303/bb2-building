#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use fermi::use_read;
use data::term::nodes_to_string;

use crate::atoms::DATABASE;
use crate::components::sprite::Sprite;
use crate::pages::Route;

#[component]
pub fn SkillView<'a>(cx: Scope<'a>, skill: &'a data::skill::Skill) -> Element {
    let database = use_read(cx, &DATABASE);
    render! {
        div {
            class: "flex flex-col border-solid border border-base-300 rounded-md my-2",
            div {
                class: "flex flex-row items-center gap-2 bg-base-300 text-base-content p-2",
                Sprite { sprite: &skill.modes[0].icon }
                span {
                    class: "flex-grow",
                    Link {
                        class: "text-primary hover:underline cursor-pointer",
                        to: Route::SkillPage { skill_id: skill.id.clone() },
                        skill.name(database.term())
                    }
                }
                span {
                    Rarity { rarity: skill.rarity }
                }
            }
            ul {
                class: "flex flex-row gap-2 p-2",
                for mode in &skill.modes {
                    li {
                        SkillMode { mode: mode }
                    }
                }
            }
        }
     }
}

#[component]
pub fn SkillMode<'a>(cx: Scope<'a>, mode: &'a data::skill::SkillMode) -> Element {
    let database = use_read(cx, &DATABASE);

    let nodes = mode.format(database);

    render! {
        div { class: "flex flex-col gap-2 bg-base-200 text-base-content rounded-md p-2",
            div { class: "flex flex-row items-center gap-2",
                Sprite { sprite: &mode.icon }
                div { class: "flex-grow",
                    mode.name(database.term())
                }
                div { class: "dropdown",
                    div { class: "btn btn-ghost btn-circle btn-sm",
                        tabindex: 0,
                        role: "button",
                        dangerous_inner_html: r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M6.75 12a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM12.75 12a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM18.75 12a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0Z" /></svg>"#,
                    }
                    ul { class: "menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52",
                        tabindex: 0,
                        li {
                            button {
                                class: "btn btn-ghost btn-sm justify-start",
                                onclick: move |_| log::info!("{:#?}", mode),
                                "Dump"
                            }
                        }
                    }
                }
            }
            div {
                Description { nodes: nodes.clone() }
            }
        }
    }
}

#[component]
fn Rarity(cx: Scope, rarity: i8) -> Element {
    let db = use_read(cx, &DATABASE);
    let color = db.term().tr(format!("CLR-Star-Rarity-{}", rarity).as_str(), |n| nodes_to_string(n));
    render! {
        div {
            class: "flex flex-row",
            color: "#{color}",
            for _ in 0..(*rarity) {
                svg {
                    class: "w-4 h-4",
                    view_box: "0 0 24 24",
                    fill: "currentColor",
                    path {
                        fill_rule: "evenodd",
                        clip_rule: "evenodd",
                        d: "M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.006 5.404.434c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.434 2.082-5.005Z",
                    }
                }
            }
        }
    }
}


#[component]
pub fn Description(cx: Scope, nodes: Vec<data::term::Node>) -> Element {
    render! {
        for node in nodes {
            match node {
                data::term::Node::Text(text) => rsx! { "{text}" },
                data::term::Node::NewLine => rsx! { br {} },
                data::term::Node::Empty => rsx! { "" },
                data::term::Node::Var(name) => rsx! {
                    span { class: "text-error", "[{name}]" }
                },
                data::term::Node::Error(text) => rsx! {
                    span { class: "text-error font-bold", "{text}" }
                },
            }
        }
    }
}
