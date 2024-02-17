use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use serde::Serialize;
use typed_builder::TypedBuilder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/bundle.js")]
extern "C" {
    #[wasm_bindgen(js_name = createAuth0)]
    async fn create_auth0() -> JsValue;

    type Auth0Client;

    #[wasm_bindgen(method, js_name = isAuthenticated)]
    async fn is_authenticated(this: &Auth0Client) -> JsValue;

    #[wasm_bindgen(method, js_name = loginWithRedirect)]
    async fn login_with_redirect(this: &Auth0Client, options: JsValue);

    #[wasm_bindgen(method, js_name = handleRedirectCallback)]
    async fn handle_redirect_callback(this: &Auth0Client);

    #[wasm_bindgen(method)]
    async fn logout(this: &Auth0Client, options: JsValue);
}

#[derive(Default, Clone, Serialize, TypedBuilder)]
pub struct RedirectLoginOptions {
    // appState?: TAppState;
    #[builder(default, setter(strip_option))]
    #[serde(rename = "authorizationParams")]
    authorization_params: Option<AuthorizationParams>,
    #[builder(default, setter(strip_option))]
    fragment: Option<String>,
    // onRedirect?: ((url) => Promise<void>);
    // openUrl?: ((url) => void | Promise<void>);
}

#[derive(Default, Clone, Serialize, TypedBuilder)]
pub struct AuthorizationParams {
    // pub acr_values: Option<String>,
    /// The default audience to be used for requesting API access.
    // pub audience: Option<String>,
    /// The name of the connection configured for your application. If null, it will redirect to the Auth0 Login Page and show the Login Widget.
    // pub connection: Option<String>,
    // display?: "page" | "popup" | "touch" | "wap";
    /// Previously issued ID Token.
    // id_token_hint?: string;
    /// The Id of an invitation to accept. This is available from the user invitation URL that is given when participating in a user invitation flow.
    // invitation?: string;
    /// The user's email address or other identifier. When your app knows which user is trying to authenticate, you can provide this parameter to pre-fill the email box or select the right session for sign-in.
    ///
    /// This currently only affects the classic Lock experience.
    // login_hint?: string;
    /// Maximum allowable elapsed time (in seconds) since authentication. If the last time the user authenticated is greater than this value, the user must be reauthenticated.
    // max_age?: string | number;
    /// The organization to log in to.
    // organization?: string;
    // prompt?: "none" | "login" | "consent" | "select_account";
    /// The default URL where Auth0 will redirect your browser to with the authentication result. It must be whitelisted in the "Allowed Callback URLs" field in your Auth0 Application's settings. If not provided here, it should be provided in the other methods that provide authentication.
    #[builder(default, setter(strip_option))]
    redirect_uri: Option<String>,
    // scope?: string;
    // screen_hint?: string;
    // ui_locales?: string;
    // [key: string]: any;
}

#[derive(Default, Clone, Serialize, TypedBuilder)]
pub struct LogoutOptions {
    #[builder(default, setter(strip_option))]
    #[serde(rename = "clientId")]
    client_id: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "logoutParams")]
    logout_params: Option<LogoutParams>,
    // onRedirect?: ((url) => Promise<void>);
    // openUrl?: ((url) => void | Promise<void>);
}

#[derive(Default, Clone, Serialize, TypedBuilder)]
pub struct LogoutParams {
    #[builder(default, setter(strip_option))]
    federated: Option<bool>,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "returnTo")]
    return_to: Option<String>,
}

enum Action {
    Login(RedirectLoginOptions),
    HandleRedirectCallback,
    Logout(LogoutOptions),
}

#[derive(Copy, Clone)]
struct Auth0Context {
    is_authenticated: Signal<bool>,
}

pub fn use_auth0() -> UseAuth0 {
    let mut is_authenticated = use_signal(|| false);
    let context = Auth0Context { is_authenticated };
    let context = use_context_provider(|| context);

    let channel = use_coroutine(|mut rx| async move {
        let client: Auth0Client = create_auth0().await.into();

        while let Some(action) = rx.next().await {
            match action {
                Action::Login(options) => {
                    let object = serde_wasm_bindgen::to_value(&options)
                        .expect("failed to serialize options");
                    client.login_with_redirect(object).await;
                }
                Action::HandleRedirectCallback => {
                    client.handle_redirect_callback().await;
                }
                Action::Logout(options) => {
                    let object = serde_wasm_bindgen::to_value(&options)
                        .expect("failed to serialize options");
                    client.logout(object).await;
                }
            }
            *is_authenticated.write() = client.is_authenticated().await.as_bool().unwrap_or(false);
        }
    });

    let auth0 = UseAuth0 { context, channel };

    use_effect(move || {
        let pathname = web_sys::window().unwrap().location().pathname().unwrap();
        tracing::info!("pathname: {}", pathname);
        if pathname.starts_with("/auth/callback") {
            auth0.handle_redirect_callback();
        }
    });

    auth0
}

pub fn use_auth0_context() -> UseAuth0 {
    let context = use_context::<Auth0Context>();
    let channel = use_coroutine_handle::<Action>();

    UseAuth0 { context, channel }
}

#[derive(Copy, Clone)]
pub struct UseAuth0 {
    context: Auth0Context,
    channel: Coroutine<Action>,
}

impl UseAuth0 {
    pub fn is_authenticated(&self) -> Signal<bool> {
        self.context.is_authenticated
    }

    pub fn login_with_redirect(&self, options: RedirectLoginOptions) {
        self.channel.send(Action::Login(options))
    }

    pub fn handle_redirect_callback(&self) {
        self.channel.send(Action::HandleRedirectCallback)
    }

    pub fn logout(&self, options: LogoutOptions) {
        self.channel.send(Action::Logout(options))
    }
}
