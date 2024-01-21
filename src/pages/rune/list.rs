use std::fmt::Display;
use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_signals::Signal;
use fermi::use_read;
use serde::{Deserialize, Serialize};

use crate::atoms::DATABASE;
use crate::components::RuneView;
use crate::hooks::use_search_rune;
use crate::pages::Route;

#[derive(Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RuneListQuery {
    query: Signal<String>,
}

impl FromStr for RuneListQuery {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for RuneListQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(s) => f.write_str(&s),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

#[component]
pub fn RuneListPage(cx: Scope, query: RuneListQuery) -> Element {
    let db = use_read(cx, &DATABASE);

    let search = use_search_rune(cx);
    if *search.query.peek() != *query.query.peek() {
        search.query.set(query.query.peek().clone());
    }

    render! {
        div { class: "text-sm breadcrumbs",
            ul {
                li { "Home" }
                li { "Rune" }
            }
        }

        div { class: "flex flex-row items-center gap-4",
            div { class: "relative flex-grow",
                input { class: "input input-bordered input-primary w-full",
                    r#type: "text",
                    placeholder: "Search skills...",
                    autofocus: true,
                    value: "{query.query}",
                    oninput: move |e| {
                        let q = e.data.value();
                        search.query.set(q.clone());
                        let router = dioxus_router::router();
                        router.replace(Route::RuneListPage {
                            query: RuneListQuery {
                                query: Signal::new(q.clone()),
                            },
                        });
                    }
                }
            }
            div { class: "badge badge-accent badge-lg gap-1 text-xs",
                span { class: "font-bold",
                    "{search.results.read().len()}"
                }
                span {
                    "of"
                }
                span { class: "font-bold",
                    "{db.rune.iter().count()}"
                }
            }
        }

        div { class: "flex flex-wrap gap-2 mt-4",
            for rune in search.results.read().iter() {
                div { class: "flex-1 min-w-64",
                    RuneView { rune: rune.clone() }
                }
            }
        }
    }
}
