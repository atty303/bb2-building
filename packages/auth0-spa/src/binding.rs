use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/bundle.mjs")]
extern "C" {
    #[wasm_bindgen(js_name = createAuth0Client)]
    pub async fn create_auth0_client(options: JsValue) -> JsValue;

    pub type Auth0Client;

    #[wasm_bindgen(constructor)]
    pub fn new(options: JsValue) -> Auth0Client;

    #[wasm_bindgen(method, js_name = isAuthenticated)]
    pub async fn is_authenticated(this: &Auth0Client) -> JsValue;

    #[wasm_bindgen(method, js_name = loginWithRedirect)]
    pub async fn login_with_redirect(this: &Auth0Client, options: JsValue);

    #[wasm_bindgen(method, js_name = handleRedirectCallback)]
    pub async fn handle_redirect_callback(this: &Auth0Client) -> JsValue;

    #[wasm_bindgen(method)]
    pub async fn logout(this: &Auth0Client, options: JsValue);
}

pub mod structs {
    use super::*;

    #[derive(Default, Clone, Serialize, TypedBuilder)]
    pub struct Auth0ClientOptions {
        /// Internal property to send information about the client to the authorization server.
        #[builder(default, setter(strip_option))]
        #[serde(rename = "auth0Client")]
        pub auth0_client: Option<Auth0ClientInternal>,

        /// URL parameters that will be sent back to the Authorization Server.
        /// This can be known parameters defined by Auth0 or custom parameters that you define.
        #[builder(default, setter(strip_option))]
        #[serde(rename = "authorizationParams")]
        pub authorization_params: Option<AuthorizationParams>,

        /// A maximum number of seconds to wait before declaring background calls to /authorize as failed for timeout Defaults to 60s.
        #[builder(default, setter(strip_option))]
        #[serde(rename = "authorizeTimeoutInSeconds")]
        pub authorize_timeout_in_seconds: Option<u32>,

        // cache?: ICache
        /// The location to use when storing cache data. The default setting is `CacheLocation::Memory`.
        ///
        /// Read more about [changing storage options in the Auth0 docs](https://auth0.com/docs/libraries/auth0-single-page-app-sdk#change-storage-options)
        #[builder(default, setter(strip_option))]
        #[serde(rename = "cacheLocation")]
        pub cache_location: Option<CacheLocation>,

        /// The Client ID found on your Application settings page
        #[builder(default, setter(strip_option))]
        #[serde(rename = "clientId")]
        pub client_id: Option<String>,

        /// The domain the cookie is accessible from. If not set, the cookie is scoped to the current domain, including the subdomain.
        ///
        /// Note: setting this incorrectly may cause silent authentication to stop working on page load.
        ///
        /// To keep a user logged in across multiple subdomains set this to your top-level domain and prefixed with a `.` (eg: `.example.com`).
        #[builder(default, setter(strip_option))]
        #[serde(rename = "cookieDomain")]
        pub cookie_domain: Option<String>,

        /// Your Auth0 account domain such as `'example.auth0.com'`, `'example.eu.auth0.com'` or , `'example.mycompany.com'` (when using [custom domains](https://auth0.com/docs/custom-domains))
        pub domain: String,

        /// Specify the timeout for HTTP calls using `fetch`. The default is 10 seconds.
        #[builder(default, setter(strip_option))]
        #[serde(rename = "httpTimeoutInSeconds")]
        pub http_timeout_in_seconds: Option<u32>,

        /// The issuer to be used for validation of JWTs, optionally defaults to the domain above
        #[builder(default, setter(strip_option))]
        pub issuer: Option<String>,

        /// The value in seconds used to account for clock skew in JWT expirations. Typically, this value is no more than a minute or two at maximum. Defaults to 60s.
        #[builder(default, setter(strip_option))]
        pub leeway: Option<u32>,

        /// Sets an additional cookie with no SameSite attribute to support legacy browsers that are not compatible with the latest SameSite changes.
        /// This will log a warning on modern browsers, you can disable the warning by setting this to false but be aware that some older useragents will not work, See <https://www.chromium.org/updates/same-site/incompatible-clients>
        /// Defaults to true
        #[builder(default, setter(strip_option))]
        pub legacy_same_site_cookie: Option<bool>,

        // nowProvider?: (() => number | Promise<number>)
        /// Number of days until the cookie `auth0.is.authenticated` will expire Defaults to 1.
        #[builder(default, setter(strip_option))]
        #[serde(rename = "sessionCheckExpiryDays")]
        pub session_check_expiry_days: Option<u32>,

        /// If true, the SDK will use a cookie when storing information about the auth transaction while the user is going through the authentication flow on the authorization server.
        ///
        /// The default is `false`, in which case the SDK will use session storage.
        ///
        /// **Note:** You might want to enable this if you rely on your users being able to authenticate using flows that may end up spanning across multiple tabs (e.g. magic links) or you cannot otherwise rely on session storage being available.
        #[builder(default, setter(strip_option))]
        #[serde(rename = "useCookiesForTransactions")]
        pub use_cookies_for_transactions: Option<bool>,

        /// If `true`, data to the token endpoint is transmitted as x-www-form-urlencoded data, if false it will be transmitted as JSON. The default setting is `true`.
        ///
        /// **Note:** Setting this to `false` may affect you if you use Auth0 Rules and are sending custom, non-primitive data. If you disable this, please verify that your Auth0 Rules continue to work as intended.
        #[builder(default, setter(strip_option))]
        #[serde(rename = "useFormData")]
        pub use_form_data: Option<bool>,

        /// If true, refresh tokens are used to fetch new access tokens from the Auth0 server. If false, the legacy technique of using a hidden iframe and the `authorization_code` grant with `prompt=none` is used. The default setting is `false`.
        ///
        /// **Note:** Use of refresh tokens must be enabled by an administrator on your Auth0 client application.
        #[builder(default, setter(strip_option))]
        #[serde(rename = "useRefreshTokens")]
        pub use_refresh_tokens: Option<bool>,

        /// If true, fallback to the technique of using a hidden iframe and the `authorization_code` grant with `prompt=none` when unable to use refresh tokens. If false, the iframe fallback is not used and errors relating to a failed `refresh_token` grant should be handled appropriately. The default setting is `false`.
        ///
        /// **Note:** There might be situations where doing silent auth with a Web Message response from an iframe is not possible, like when you're serving your application from the file system or a custom protocol (like in a Desktop or Native app). In situations like this you can disable the iframe fallback and handle the failed `refresh_token` grant and prompt the user to login interactively with `loginWithRedirect` or `loginWithPopup`."
        ///
        /// E.g. Using the `file:` protocol in an Electron application does not support that legacy technique.
        ///
        /// # Example
        ///
        /// ```typescript
        /// let token: string;
        /// try {
        ///   token = await auth0.getTokenSilently();
        /// } catch (e) {
        ///   if (e.error === 'missing_refresh_token' || e.error === 'invalid_grant') {
        ///     auth0.loginWithRedirect();
        ///   }
        /// }
        /// ```
        #[builder(default, setter(strip_option))]
        #[serde(rename = "useRefreshTokensFallback")]
        pub use_refresh_tokens_fallback: Option<bool>,

        /// If provided, the SDK will load the token worker from this URL instead of the integrated `blob`.
        /// An example of when this is useful is if you have strict Content-Security-Policy (CSP) and wish to avoid needing to set `worker-src: blob:`.
        /// We recommend either serving the worker, which you can find in the module at `<module_path>/dist/auth0-spa-js.worker.production.js`,
        /// from the same host as your application or using the Auth0 CDN `https://cdn.auth0.com/js/auth0-spa-js/<version>/auth0-spa-js.worker.production.js`.
        ///
        /// **Note:** The worker is only used when `useRefreshTokens: true`, `cacheLocation: 'memory'`, and the `cache` is not custom.
        #[builder(default, setter(strip_option))]
        #[serde(rename = "workerUrl")]
        pub worker_url: Option<String>,
    }

    /// Internal property to send information about the client to the authorization server.
    #[derive(Default, Clone, Serialize, TypedBuilder)]
    pub struct Auth0ClientInternal {
        #[builder(default, setter(strip_option))]
        pub env: Option<HashMap<String, String>>,
        pub name: String,
        pub version: String,
    }

    #[derive(Clone, Serialize)]
    pub enum CacheLocation {
        #[serde(rename = "memory")]
        Memory,
        #[serde(rename = "localstorage")]
        LocalStorage,
    }

    #[derive(Default, Clone, Serialize, TypedBuilder)]
    pub struct RedirectLoginOptions<TAppState: Serialize> {
        #[builder(default, setter(strip_option))]
        #[serde(rename = "appState")]
        pub app_state: Option<TAppState>,
        #[builder(default, setter(strip_option))]
        #[serde(rename = "authorizationParams")]
        pub authorization_params: Option<AuthorizationParams>,
        #[builder(default, setter(strip_option))]
        pub fragment: Option<String>,
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

    #[derive(Deserialize)]
    pub struct RedirectLoginResult<TAppState> {
        /// State stored when the redirect request was made
        #[serde(rename = "appState")]
        pub app_state: Option<TAppState>,
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
}
