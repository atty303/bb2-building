#![allow(non_snake_case)]

use anyhow::anyhow;
use dioxus::prelude::*;
use dioxus_router::prelude::{Router, RouterConfig, RouterConfigFactory, WebHistory};
use fermi::{use_atom_state, use_init_atom_root, use_set};

use data::{Database, LANGUAGES};

use crate::atoms::DATABASE;
use crate::hooks::persistent::use_persistent;
use crate::pages::Route;

async fn fetch_database(lang: &str) -> anyhow::Result<Database> {
    if let Some(_) = LANGUAGES.iter().find(|l| *l == &lang) {
        let base_uri = gloo_utils::document()
            .base_uri()
            .map_err(|err| anyhow!(format!("{:?}", err)))?;
        let base_uri = base_uri.ok_or(anyhow!("base_uri"))?;

        let term_cursor = {
            let res = reqwest::get(format!("{}i18n/{}/term.msgpack", base_uri, lang)).await?;
            let body = res.bytes().await?;
            std::io::Cursor::new(body)
        };
        let skill_cursor = {
            let res = reqwest::get(format!("{}i18n/{}/skill.msgpack", base_uri, lang)).await?;
            let body = res.bytes().await?;
            std::io::Cursor::new(body)
        };

        Database::read(term_cursor, skill_cursor).map_err(|err| anyhow!(err))
    } else {
        Err(anyhow!("unknown language: {}", lang))
    }
}

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    let language = use_atom_state(cx, &crate::atoms::LANGUAGE);

    let language_persistent = use_persistent(cx, "language", || "en".to_string());
    use_effect(cx, &language_persistent.get(), move |lang| {
        to_owned![language];
        async move {
            if lang != "" {
                language.set(lang);
            }
        }
    });

    use_effect(cx, language.get(), move |lang| {
        to_owned![language_persistent];
        async move {
            // if language_persistent.get() != lang {
            language_persistent.set(lang.to_string());
            // }
        }
    });

    let set_database = use_set(cx, &DATABASE);
    let database_future = use_future(cx, language, |_| {
        to_owned![language, set_database];
        async move {
            if language.get() != "" {
                let mut db = fetch_database(&language).await;
                match db {
                    Ok(ref mut v) => {
                        set_database(v.clone());
                    }
                    _ => (),
                }
                Some(db)
            } else {
                None
            }
        }
    });

    match database_future.value() {
        Some(Some(Ok(_))) => {
            render! {
                Router::<Route> {
                    config: RouterConfigFactory::from(|| RouterConfig::default().history(WebHistory::<Route>::default())),
                }
            }
        }
        Some(Some(Err(err))) => {
            render! {
                "An error occurred while fetching database: {err}"
            }
        }
        Some(None) => {
            render! {
                ""
            }
        }
        None => {
            // Loading
            render! {
                ""
            }
        }
    }
}
