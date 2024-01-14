#![allow(non_snake_case)]

use anyhow::anyhow;
use dioxus::prelude::*;

use data::Database;
use data::term::TermMap;

use crate::components::skill_view::SkillView;

async fn fetch_database() -> anyhow::Result<Database> {
    let base_uri = gloo_utils::document().base_uri().map_err(|err| anyhow!(format!("{:?}", err)))?;
    let base_uri = base_uri.ok_or(anyhow!("base_uri"))?;

    let res = reqwest::get(base_uri + "data/skill.avro").await?;
    let body = res.bytes().await?;
    let cursor = std::io::Cursor::new(body);

    Database::read(cursor).map_err(|err| anyhow!(err))
}

async fn fetch_i18n(lang: &str) -> anyhow::Result<TermMap> {
    let base_uri = gloo_utils::document().base_uri().map_err(|err| anyhow!(format!("{:?}", err)))?;
    let base_uri = base_uri.ok_or(anyhow!("base_uri"))?;

    let res = reqwest::get(format!("{}i18n/{}/terms.avro", base_uri, lang)).await?;
    let body = res.bytes().await?;
    let cursor = std::io::Cursor::new(body);

    TermMap::read(cursor).map_err(|err| anyhow!(err))
}

pub fn App<'a>(cx: Scope<'a>) -> Element<'a> {
    let lang = use_state(cx, || "ja");

    use_shared_state_provider(cx, || Database::default());
    let database_state = use_shared_state::<Database>(cx).unwrap();
    let database_future = use_future(cx, lang, |_| {
        to_owned![lang, database_state];
        async move {
            let i18n = fetch_i18n(*lang).await?;
            let mut db = fetch_database().await;
            match db {
                Ok(ref mut v) => {
                    v.set_term(i18n);
                    *database_state.write() = v.clone()
                },
                _ => ()
            }
            db
        }
    });

    match database_future.value() {
        Some(Ok(database)) => {
            render! {
                for skill in database.skill.values().filter(|skill| skill.id == "SKA01010") {
                    SkillView { skill: skill }
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