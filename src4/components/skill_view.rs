use dioxus::html::geometry::euclid::Rect;
use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use dioxus_signals::{use_selector, use_signal, Signal};
use fermi::use_read;

use data::token::{Token, Tokens};

use crate::atoms::DATABASE;
use crate::components::{Rarity, SpriteIcon};
use crate::pages::Route;

#[component]
pub fn SkillView(
    cx: Scope,
    skill: Signal<data::skill::Skill>,
    #[props(default = false)] debug: bool,
) -> Element {
    render! {
        div {
            class: "flex flex-col border-solid border border-base-300 rounded-md my-2",
            div {
                class: "flex flex-row items-center gap-2 bg-base-300 text-base-content p-2",
                SpriteIcon { class: "rounded-md", sprite: Signal::new(skill.read().modes[0].icon.clone()), size: 48 }
                span {
                    class: "flex-grow",
                    Link {
                        class: "text-primary hover:underline cursor-pointer",
                        to: Route::SkillPage { skill_id: skill.read().id.clone() },
                        "{skill.read().name}"
                    }
                }
                span {
                    Rarity { rarity: skill.read().rarity }
                }
            }
            div { class: "flex flex-row flex-wrap gap-2 p-2",
                for mode in skill.read().modes.iter() {
                    div { class: "flex-1 min-w-64",
                        SkillMode { mode: Signal::new(mode.clone()), debug: *debug }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SkillMode(
    cx: Scope,
    mode: Signal<data::skill::SkillMode>,
    #[props(default = false)] debug: bool,
) -> Element {
    render! {
        div { class: "flex flex-col gap-2 bg-base-200 text-base-content rounded-md p-2",
            div { class: "flex flex-row items-center gap-2",
                SpriteIcon { class: "rounded-md", sprite: Signal::new(mode.read().icon.clone()), size: 32 }
                div { class: "flex-grow",
                    "{mode.read().name}"
                }
            }
            div { class: "bg-base-100 p-2",
                Description { tokens: mode.read().format(), debug: *debug }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Node {
    Text(String),
    NewLine,
    Var(String),
    Error(String),
    Indent,
    Term {
        name: String,
        tips: Option<String>,
        children: Vec<Node>,
    },
}

fn to_nodes(tokens: &Tokens) -> Vec<Node> {
    let mut stack = vec![(None, vec![])];
    for token in tokens.vec() {
        let node = match token {
            Token::Text(text) => Some(Node::Text(text.clone())),
            Token::NewLine => Some(Node::NewLine),
            Token::Empty => None,
            Token::Var(name) => Some(Node::Var(name.clone())),
            Token::Error(text) => Some(Node::Error(text.clone())),
            Token::Indent => Some(Node::Indent),
            Token::Panic(text) => Some(Node::Error(text.clone())),
            n @ Token::TermStart(_, _) => {
                stack.push((Some(n.clone()), vec![]));
                None
            }
            Token::TermEnd => {
                let (n, last) = stack.pop().unwrap();
                if let Token::TermStart(name, tips) = n.unwrap() {
                    Some(Node::Term {
                        name: name.clone(),
                        tips: tips.clone(),
                        children: last,
                    })
                } else {
                    None
                }
            }
        };
        if node.is_some() {
            stack.last_mut().unwrap().1.push(node.unwrap());
        }
    }
    stack.pop().unwrap().1
}

#[component]
fn RenderNode(cx: Scope, node: Node, #[props(default = false)] debug: bool) -> Element {
    match node {
        Node::Text(text) => render! { "{text}" },
        Node::NewLine => render! { br {} },
        Node::Var(name) => render! {
            span { class: "text-error", "[{name}]" }
        },
        Node::Error(text) => render! {
            span { class: "text-error font-bold", "{text}" }
        },
        Node::Indent => render! {
            br {}
            "　"
        },
        Node::Term {
            name,
            tips,
            children,
        } => {
            let debug_class = if *debug {
                "border border-secondary rounded p-1 m-1"
            } else {
                ""
            };
            let title = if *debug { name.clone() } else { "".to_string() };
            if let Some(tips) = tips {
                render! {
                    span { class: "{debug_class} inline-block border-b-2 border-primary border-dotted",
                        Tooltip { name: tips.clone(),
                            span { class: "text-primary",
                                title: "{title}",
                                for node in &children {
                                    RenderNode { node: node.clone(), debug: *debug }
                                }
                            }
                        }
                    }
                }
            } else {
                render! {
                    span { class: "{debug_class} inline-block",
                        span {
                            title: "{title}",
                            for node in &children {
                                RenderNode { node: node.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Description(cx: Scope, tokens: Tokens, #[props(default = false)] debug: bool) -> Element {
    let nodes = to_nodes(tokens);

    render! {
        for node in &nodes {
            RenderNode { node: node.clone(), debug: *debug }
        }
    }
}

#[component]
pub fn Tooltip<'a>(
    cx: Scope<'a>,
    name: String,
    #[props(default = false)] debug: bool,
    children: Element<'a>,
) -> Element {
    let db = use_read(cx, &DATABASE);

    let title = db.term.get(&format!("NM-{}", name));
    let body = db.term.get(&format!("DC-{}", name));

    // let open = use_signal(cx, || false);
    let popover_position = use_signal(cx, || None::<Rect<f64, f64>>);
    let popover_style = use_selector(cx, move || {
        if let Some(r) = *popover_position.read() {
            if r.origin.x < 0.0 {
                format!("position: absolute; right: {}px;", r.origin.x)
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
    });

    render! {
        div { class: "dropdown dropdown-end",
            div { class: "",
                tabindex: 0,
                role: "button",
                {children}
            }
            div { class: "dropdown-content z-[1] card card-compact card-bordered border-base-300 shadow-lg shadow-black/50 bg-base-100 text-base-content min-w-64 max-w-full",
                tabindex: 0,
                style: "{popover_style}",
                onmounted: move |e| {
                    async move {
                        let r = e.data.get_client_rect().await;
                        popover_position.set(Some(r.unwrap()));
                    }
                },
                div { class: "card-body",
                    span { class: "font-bold",
                        Description { tokens: title, debug: *debug }
                    }
                    Description { tokens: body, debug: *debug }
                }
            }
        }
    }
}
