use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/bundle.js")]
extern "C" {
    #[wasm_bindgen(js_name = createAuth0)]
    async fn create_auth0() -> JsValue;

    type Auth0Client;

    #[wasm_bindgen(method, js_name = isAuthenticated)]
    async fn is_authenticated(this: &Auth0Client) -> JsValue;
}

pub fn use_auth0() -> UseAuth0 {
    let client: Resource<Auth0Client> = use_resource(|| async move {
        let client = create_auth0().await;
        client.into()
    });

    UseAuth0 {
        client: client.value(),
    }
}

#[derive(Copy, Clone)]
pub struct UseAuth0 {
    client: ReadOnlySignal<Option<Auth0Client>>,
}

impl UseAuth0 {
    pub async fn is_authenticated(&self) -> bool {
        if let Some(ref client) = *self.client.read() {
            client.is_authenticated().await.as_bool().unwrap_or(false)
        } else {
            false
        }
    }
}
