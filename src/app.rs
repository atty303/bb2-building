use anyhow::anyhow;
use dioxus::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::{Router, RouterConfig, RouterConfigFactory, WebHistory};

use crate::global::{DATABASE, LANGUAGE, SEARCH_CATALOGS, THEME};
use crate::hooks::{use_on_create, use_persistent};
use crate::pages::Route;
use data::{Database, LANGUAGES};

use crate::search::{RuneSearch, SearchCatalogs, SkillSearch};

use crate::ui::{Dialog, DialogPanel};

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
    let theme_persistent = use_persistent("theme", || "dark".to_string());
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

    let language_persistent = use_persistent("language", || "en".to_string());
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
            let mut db = fetch_database(&lang).await;
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

    match *database_future.value().read() {
        Some(Some(Ok(_))) => {
            rsx! {
                Router::<Route> {
                    config: RouterConfigFactory::from(|| RouterConfig::default().history(WebHistory::<Route>::default())),
                }
            }
        }
        Some(Some(Err(ref err))) => {
            rsx! {
                "An error occurred while fetching database: {err}"
            }
        }
        Some(None) | None => {
            // While loading database
            rsx! {
                ""
            }
        }
    }
}

#[component]
fn DaisyDialog(children: Element) -> Element {
    let mut open = use_signal(|| true);

    rsx! {
        Dialog {
            open: open(),
            on_close: move |_| {
                open.set(false);
            },
            // render: RenderFn::new(|args: DialogRenderArgs| {
            //     rsx! {
            //         dialog {
            //             ..args.attrs,
            //             {args.children}
            //         }
            //     }
            // }),
            DialogPanel {
                {children}
            }
        }
    }
}
