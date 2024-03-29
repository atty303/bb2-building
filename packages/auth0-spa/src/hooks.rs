use crate::binding::{create_auth0_client, Auth0Client};
use crate::{
    Auth0ClientOptions, GetTokenSilentlyOptions, GetTokenSilentlyVerboseResponse, LogoutOptions,
    RedirectLoginOptions, RedirectLoginResult,
};
use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};

enum Action<TAppState: Serialize> {
    GetTokenSilently(GetTokenSilentlyOptions),
    HandleRedirectCallback,
    LoginWithRedirect(RedirectLoginOptions<TAppState>),
    Logout(LogoutOptions),
}

#[derive(Copy, Clone)]
struct Auth0Context {
    is_authenticated: Signal<bool>,
    token: Signal<Option<GetTokenSilentlyVerboseResponse>>,
}

pub fn use_auth0<TAppState: Clone + Serialize + for<'de> Deserialize<'de>>(
    options: Auth0ClientOptions,
    mut redirect_callback: impl FnMut(RedirectLoginResult<TAppState>) + 'static,
) -> UseAuth0<TAppState> {
    let mut is_authenticated = use_signal(|| false);
    let mut token = use_signal(|| None);
    let context = Auth0Context {
        is_authenticated,
        token,
    };
    let context = use_context_provider(|| context);

    let channel = use_coroutine(move |mut rx| async move {
        let object = serde_wasm_bindgen::to_value(&options).expect("failed to serialize options");
        let client: Auth0Client = create_auth0_client(object).await.into();
        *is_authenticated.write() = client.is_authenticated().await.as_bool().unwrap_or(false);

        while let Some(action) = rx.next().await {
            match action {
                Action::GetTokenSilently(options) => {
                    let object = serde_wasm_bindgen::to_value(&options)
                        .expect("failed to serialize options");
                    let result_js = client.get_token_silently(object).await;
                    tracing::info!("got token: {:?}", result_js);
                    let result = serde_wasm_bindgen::from_value::<GetTokenSilentlyVerboseResponse>(
                        result_js.clone(),
                    );
                    if let Ok(ok) = result {
                        tracing::info!("got token: {:?}", ok);
                        *token.write() = Some(ok);
                    } else {
                        tracing::error!("failed to deserialize result: {:?}", result_js);
                    }
                }
                Action::HandleRedirectCallback => {
                    let result_js = client.handle_redirect_callback().await;
                    let result = serde_wasm_bindgen::from_value::<RedirectLoginResult<TAppState>>(
                        result_js.clone(),
                    );
                    if let Ok(ok) = result {
                        redirect_callback(ok);
                    } else {
                        tracing::error!(
                            "failed to deserialize redirect login result: {:?}",
                            result_js
                        );
                    }
                }
                Action::LoginWithRedirect(options) => {
                    let object = serde_wasm_bindgen::to_value(&options)
                        .expect("failed to serialize options");
                    client.login_with_redirect(object).await;
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

    UseAuth0 { context, channel }
}

pub fn use_auth0_context<TAppState: Clone + Serialize>() -> UseAuth0<TAppState> {
    let context = use_context::<Auth0Context>();
    let channel = use_coroutine_handle::<Action<TAppState>>();

    UseAuth0 { context, channel }
}

#[derive(Copy, Clone)]
pub struct UseAuth0<TAppState: Clone + Serialize + 'static> {
    context: Auth0Context,
    channel: Coroutine<Action<TAppState>>,
}

impl<TAppState: Clone + Serialize> UseAuth0<TAppState> {
    /// Returns true if there's valid information stored, otherwise returns false.
    pub fn is_authenticated(&self) -> Signal<bool> {
        self.context.is_authenticated
    }

    pub fn get_token_silently(&self, options: GetTokenSilentlyOptions) {
        self.channel.send(Action::GetTokenSilently(options))
    }

    pub fn handle_redirect_callback(&self) {
        self.channel.send(Action::HandleRedirectCallback)
    }

    /// Performs a redirect to `/authorize` using the parameters provided as arguments.
    /// Random and secure state and nonce parameters will be auto-generated.
    pub fn login_with_redirect(&self, options: RedirectLoginOptions<TAppState>) {
        self.channel.send(Action::LoginWithRedirect(options))
    }

    pub fn logout(&self, options: LogoutOptions) {
        self.channel.send(Action::Logout(options))
    }
}
