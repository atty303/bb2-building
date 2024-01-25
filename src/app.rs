use anyhow::anyhow;
use dioxus::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::{Router, RouterConfig, RouterConfigFactory, WebHistory};

use crate::global::LANGUAGE;
use crate::hooks::{use_on_create, use_persistent};
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
    // TODO: use_config
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

    rsx! {
        div { class: "w-full h-full",
            button { class: "btn btn-primary",
                onclick: move |_| {
                    tracing::info!("clicked");
                },
                "Click me!"
            }
        }
        DaisyDialog {
            div { class: "font-bold",
                "Hello world!"
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
