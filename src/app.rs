use dioxus::prelude::*;
use dioxus_router::prelude::{Router, RouterConfig, RouterConfigFactory, WebHistory};

use auth0_spa::{use_auth0, Auth0ClientOptions, CacheLocation};

use crate::global::THEME;
use crate::hooks::{use_on_create, use_persistent};
use crate::pages::Route;

#[component]
pub fn App() -> Element {
    let _ = use_auth0::<String>(
        Auth0ClientOptions::builder()
            .domain("bb2b.us.auth0.com".to_string())
            .client_id("udNY8zDu6nALh3lQFJaYykONTiJgGob1".to_string())
            .cache_location(CacheLocation::LocalStorage)
            .use_refresh_tokens(true)
            .legacy_same_site_cookie(false)
            .build(),
        |r| {
            tracing::info!("{:?}", r.app_state);
            if let Some(url) = r.app_state {
                web_sys::window()
                    .unwrap()
                    .location()
                    .set_href(&url)
                    .unwrap();
            }
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

    rsx! {
        Router::<Route> {
            config: RouterConfigFactory::from(|| {
                RouterConfig::default().history(WebHistory::<Route>::default())
            })
        }
    }
}
