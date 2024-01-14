#![allow(non_snake_case)]

use anyhow::anyhow;
use dioxus::prelude::*;

#[derive(Clone, Debug, Default)]
struct Database {
}

async fn fetch_database(lang: &str) -> anyhow::Result<Database> {
    let base_uri = gloo_utils::document().base_uri().map_err(|err| anyhow!(format!("{:?}", err)))?;
    let base_uri = base_uri.ok_or(anyhow!("base_uri"))?;

    let res = reqwest::get(base_uri + "data/skill.avro").await?;
    let body = res.bytes().await?;

    Ok(Database {})
}

pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Database::default());
    let database_state = use_shared_state::<Database>(cx).unwrap();
    let database_future = use_future(cx, (), |_| {
        to_owned![database_state];
        async move {
            let db = fetch_database("ja").await;
            match db {
                Ok(ref v) => *database_state.write() = v.clone(),
                _ => ()
            }
            db
        }
    });

    match database_future.value() {
        Some(Ok(database)) => {
            render! {
                h1 { "Hello, World!" }
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