use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use dioxus_signals::Signal;

use data::token::{Token, Tokens};

use crate::components::{Rarity, SpriteIcon};
use crate::pages::Route;

#[component]
pub fn SkillView<'a>(cx: Scope, skill: &'a data::skill::Skill) -> Element {
    render! {
        div {
            class: "flex flex-col border-solid border border-base-300 rounded-md my-2",
            div {
                class: "flex flex-row items-center gap-2 bg-base-300 text-base-content p-2",
                SpriteIcon { class: "rounded-md", sprite: Signal::new(skill.modes[0].icon.clone()), size: 64 }
                span {
                    class: "flex-grow",
                    Link {
                        class: "text-primary hover:underline cursor-pointer",
                        to: Route::SkillPage { skill_id: skill.id.clone() },
                        "{skill.name}"
                    }
                }
                span {
                    Rarity { rarity: skill.rarity }
                }
            }
            div { class: "flex flex-row gap-2 p-2",
                for mode in skill.modes.iter().filter(|_m| true) {
                    div { class: "flex-1",
                        SkillMode { mode: &mode }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SkillMode<'a>(cx: Scope<'a>, mode: &'a data::skill::SkillMode) -> Element {
    render! {
        div { class: "flex flex-col gap-2 bg-base-200 text-base-content rounded-md p-2",
            div { class: "flex flex-row items-center gap-2",
                SpriteIcon { class: "rounded-md", sprite: Signal::new(mode.icon.clone()), size: 48 }
                div { class: "flex-grow",
                    "{mode.name}"
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
                Description { tokens: mode.format() }
            }
        }
    }
}

#[component]
pub fn Description(cx: Scope, tokens: Tokens) -> Element {
    render! {
        for token in &tokens.vec() {
            match token {
                Token::Text(text) => rsx! { "{text}" },
                Token::NewLine => rsx! { br {} },
                Token::Empty => rsx! { "" },
                Token::Var(name) => rsx! {
                    span { class: "text-error", "[{name}]" }
                },
                Token::Error(text) => rsx! {
                    span { class: "text-error font-bold", "{text}" }
                },
                Token::Indent => rsx! {
                    br {}
                    "　"
                },
                Token::Panic(text) => rsx! {
                    span { class: "text-error font-bold", "{text}" }
                },
            }
        }
    }
}
