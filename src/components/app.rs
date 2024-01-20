use anyhow::anyhow;
use dioxus::prelude::*;
use dioxus_router::prelude::{Router, RouterConfig, RouterConfigFactory, WebHistory};
use fermi::{use_atom_root, use_atom_state, use_init_atom_root, use_read, use_set};

use data::{Database, LANGUAGES};

use crate::atoms::DATABASE;
use crate::hooks::use_persistent;
use crate::pages::Route;
use crate::search::{SearchCatalogs, SkillSearch};

async fn fetch_database(lang: &str) -> anyhow::Result<Database> {
    if let Some(_) = LANGUAGES.iter().find(|l| *l == &lang) {
        let base_uri = gloo_utils::document()
            .base_uri()
            .map_err(|err| anyhow!(format!("{:?}", err)))?;
        let base_uri = base_uri.ok_or(anyhow!("base_uri"))?;

        let database = {
            let res = reqwest::get(format!("{}i18n/{}/database.msgpack", base_uri, lang)).await?;
            let body = res.bytes().await?;
            let cursor = std::io::Cursor::new(body);
            Database::read(cursor)?
        };

        Ok(database)
    } else {
        Err(anyhow!("unknown language: {}", lang))
    }
}

use fermi::Writable;

#[component]
pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    // TODO: use_config
    let language = use_atom_state(cx, &crate::atoms::LANGUAGE);
    let language_persistent = use_persistent(cx, "language", || "en".to_string());
    use_effect(cx, &language_persistent.get(), move |lang| {
        to_owned![language];
        async move {
            if lang != "" {
                language.set(Some(lang));
            }
        }
    });
    use_effect(cx, language.get(), move |lang| {
        to_owned![language_persistent];
        async move {
            if let Some(inner) = lang {
                language_persistent.set(inner.clone());
                let html = gloo_utils::document_element();
                html.set_attribute("lang", &inner).unwrap();
            }
        }
    });

    let set_database = use_atom_state(cx, &DATABASE);
    let set_search_catalogs = use_atom_state(cx, &crate::atoms::SEARCH_CATALOGS);
    let database_future = use_future(cx, language, |_| {
        to_owned![language, set_database, set_search_catalogs];
        async move {
            if let Some(lang) = language.get() {
                let mut db = fetch_database(lang).await;
                match db {
                    Ok(ref mut v) => {
                        set_database.set(v.clone());
                        let db = &v.skill;
                        let catalogs = SearchCatalogs {
                            skill: crate::search::create_catalog::<
                                SkillSearch,
                                SkillSearch,
                                SkillSearch,
                            >(db.clone()),
                        };
                        set_search_catalogs.set(catalogs);
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
        Some(None) | None => {
            // While loading database
            render! {
                ""
            }
        }
    }
}
