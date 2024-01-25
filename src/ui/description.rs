use dioxus::html::geometry::euclid::Rect;
use dioxus::prelude::*;

use crate::global::DATABASE;
use data::token::{Token, Tokens};

#[component]
pub fn Description(tokens: Tokens, #[props(default = false)] debug: bool) -> Element {
    let nodes = to_nodes(&tokens);

    rsx! {
        for node in nodes {
            RenderNode { node, debug }
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
fn RenderNode(node: Node, #[props(default = false)] debug: bool) -> Element {
    match node {
        Node::Text(text) => rsx! { "{text}" },
        Node::NewLine => rsx! { br {} },
        Node::Var(name) => rsx! {
            span { class: "text-error", "[{name}]" }
        },
        Node::Error(text) => rsx! {
            span { class: "text-error font-bold", "{text}" }
        },
        Node::Indent => rsx! {
            br {}
            "　"
        },
        Node::Term {
            name,
            tips,
            children,
        } => {
            let debug_class = if debug {
                "border border-secondary rounded p-1 m-1"
            } else {
                ""
            };
            let title = if debug { name.clone() } else { "".to_string() };
            if let Some(tips) = tips {
                rsx! {
                    span { class: "{debug_class} inline-block border-b-2 border-primary border-dotted",
                        Tooltip {
                            name: tips,
                            span { class: "text-primary",
                                title,
                                for node in children {
                                    RenderNode { node, debug }
                                }
                            }
                        }
                    }
                }
            } else {
                rsx! {
                    span { class: "{debug_class} inline-block",
                        span {
                            title,
                            for node in children {
                                RenderNode { node, debug }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Tooltip(name: String, #[props(default = false)] debug: bool, children: Element) -> Element {
    let title = DATABASE().term.get(&format!("NM-{}", name));
    let body = DATABASE().term.get(&format!("DC-{}", name));

    let popover_position = use_signal(|| None::<Rect<f64, f64>>);
    let popover_style = use_memo(move || {
        if let Some(r) = popover_position() {
            if r.origin.x < 0.0 {
                format!("position: absolute; right: {}px;", r.origin.x)
            } else {
                String::default()
            }
        } else {
            String::default()
        }
    });

    rsx! {
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
                        *popover_position.write() = Some(r.unwrap());
                    }
                },
                div { class: "card-body",
                    span { class: "font-bold",
                        Description { tokens: title, debug }
                    }
                    Description { tokens: body, debug }
                }
            }
        }
    }
}
