#![allow(non_snake_case)]

use anyhow::anyhow;
use dioxus::prelude::*;
use dioxus_router::prelude::{Router, RouterConfig, RouterConfigFactory, WebHistory};
use fermi::{use_init_atom_root, use_set};

use data::term::TermRepository;
use data::Database;

use crate::atoms::DATABASE;
use crate::pages::Route;

pub struct Language {
    pub code: &'static str,
}

async fn fetch_database() -> anyhow::Result<Database> {
    let base_uri = gloo_utils::document()
        .base_uri()
        .map_err(|err| anyhow!(format!("{:?}", err)))?;
    let base_uri = base_uri.ok_or(anyhow!("base_uri"))?;

    let skill_cursor = {
        let res = reqwest::get(format!("{}{}", base_uri, "data/skill.avro")).await?;
        let body = res.bytes().await?;
        std::io::Cursor::new(body)
    };

    let state_cursor = {
        let res = reqwest::get(format!("{}{}", base_uri, "data/state.avro")).await?;
        let body = res.bytes().await?;
        std::io::Cursor::new(body)
    };

    Database::read(skill_cursor, state_cursor).map_err(|err| anyhow!(err))
}

async fn fetch_i18n(lang: &str) -> anyhow::Result<TermRepository> {
    let base_uri = gloo_utils::document()
        .base_uri()
        .map_err(|err| anyhow!(format!("{:?}", err)))?;
    let base_uri = base_uri.ok_or(anyhow!("base_uri"))?;

    let res = reqwest::get(format!("{}i18n/{}/term.avro", base_uri, lang)).await?;
    let body = res.bytes().await?;
    let cursor = std::io::Cursor::new(body);

    TermRepository::read(cursor).map_err(|err| anyhow!(err))
}

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    use_shared_state_provider(cx, || Language { code: "en" });
    let language_state = use_shared_state::<Language>(cx).unwrap();

    let set_database = use_set(cx, &DATABASE);
    let database_future = use_future(cx, language_state, |_| {
        to_owned![language_state, set_database];
        async move {
            let i18n = fetch_i18n(language_state.read().code).await?;
            let mut db = fetch_database().await;
            match db {
                Ok(ref mut v) => {
                    v.set_term(i18n);
                    set_database(v.clone());
                }
                _ => (),
            }
            db
        }
    });

    match database_future.value() {
        Some(Ok(_)) => {
            render! {
                Router::<Route> {
                    config: RouterConfigFactory::from(|| RouterConfig::default().history(WebHistory::<Route>::default())),
                }
            }
        }
        Some(Err(err)) => {
            render! {
                "An error occurred while fetching database: {err}"
            }
        }
        None => {
            render! {
                ""
            }
        }
    }
}
