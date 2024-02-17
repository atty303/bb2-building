use anyhow::anyhow;
use auth0_spa::{use_auth0, Auth0ClientOptions, CacheLocation};
use dioxus::prelude::*;
use dioxus_router::prelude::{Router, RouterConfig, RouterConfigFactory, WebHistory};

use data::{Database, LANGUAGES};

use crate::global::{DATABASE, LANGUAGE, SEARCH_CATALOGS, THEME};
use crate::hooks::{use_on_create, use_persistent};
use crate::pages::Route;
use crate::search::{RuneSearch, SearchCatalogs, SkillSearch};

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

#[component]
pub fn App() -> Element {
    let _ = use_auth0::<()>(
        Auth0ClientOptions::builder()
            .domain("bb2b.us.auth0.com".to_string())
            .client_id("udNY8zDu6nALh3lQFJaYykONTiJgGob1".to_string())
            .cache_location(CacheLocation::LocalStorage)
            .use_refresh_tokens(true)
            .legacy_same_site_cookie(false)
            .build(),
        |r| {
            tracing::info!("{:?}", r.app_state);
        },
    );

    let mut theme_persistent = use_persistent("theme", || "dark".to_string());
    use_on_create(|| {
        to_owned![theme_persistent];
        async move {
            *THEME.write() = theme_persistent.get();
        }
    });
    use_effect(move || {
        theme_persistent.set(THEME());
        gloo_utils::document_element()
            .set_attribute("data-theme", &THEME())
            .unwrap();
    });

    let mut language_persistent = use_persistent("language", || "en".to_string());
    use_on_create(|| {
        to_owned![language_persistent];
        async move {
            let lang = language_persistent.get();
            if lang != "" {
                tracing::info!("setting language to {}", lang);
                *LANGUAGE.write() = Some(lang);
            }
        }
    });
    use_effect(move || {
        if let Some(lang) = LANGUAGE() {
            language_persistent.set(lang.clone());
            gloo_utils::document_element()
                .set_attribute("lang", &lang)
                .unwrap();
        }
    });

    let database_future = use_resource(|| async move {
        if let Some(lang) = LANGUAGE() {
            let db = fetch_database(&lang).await;
            match db {
                Ok(v) => {
                    let skill = &v.skill;
                    let rune = &v.rune;
                    let catalogs = SearchCatalogs {
                        skill: crate::search::create_catalog::<SkillSearch, SkillSearch, SkillSearch>(
                            skill.clone(),
                            lang.clone(),
                        ),
                        rune: crate::search::create_catalog::<RuneSearch, RuneSearch, RuneSearch>(
                            rune.clone(),
                            lang.clone(),
                        ),
                    };

                    *SEARCH_CATALOGS.write() = catalogs;
                    *DATABASE.write() = v;
                    Some(Ok(()))
                }
                Err(e) => Some(Err(e)),
            }
        } else {
            None
        }
    });

    match database_future.value().as_ref() {
        None => None,
        Some(v) => match *v {
            None => None,
            Some(Ok(_)) => {
                rsx! {
                    Router::<Route> {
                        config: RouterConfigFactory::from(|| {
                            RouterConfig::default().history(WebHistory::<Route>::default())
                        })
                    }
                }
            }
            Some(Err(ref err)) => {
                rsx! {"An error occurred while fetching database: {err}"}
            }
        },
    }
}
