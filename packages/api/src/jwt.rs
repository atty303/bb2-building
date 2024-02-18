// use worker::*;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;
use worker::console_debug;

#[derive(Error, Debug)]
pub enum Error {
    #[error("no authorization header")]
    NoAuthorizationHeader,
    #[error("invalid authorization header")]
    InvalidAuthorizationHeader,
    #[error("no kid in header")]
    NoKid,
    #[error("fetch error: {0}")]
    FetchError(reqwest::Error),
    #[error("jwk not found: kid={0}")]
    JwkNotFound(String),
    #[error("kv error: {0}")]
    KvError(worker::kv::KvError),
    #[error("worker error: {0}")]
    WorkerError(worker::Error),
    #[error("jwt error: {0}")]
    JwtError(jsonwebtoken::errors::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
}

pub async fn verify_jwt(
    req: worker::Request,
    env: worker::Env,
) -> Result<jsonwebtoken::TokenData<Claims>, Error> {
    let auth = req
        .headers()
        .get("authorization")
        .unwrap()
        .ok_or(Error::NoAuthorizationHeader)?;
    let token = {
        let mut pair = auth.splitn(2, ' ');
        if pair.next() != Some("Bearer") {
            Err(Error::InvalidAuthorizationHeader)
        } else if let Some(token) = pair.next() {
            Ok(token)
        } else {
            Err(Error::InvalidAuthorizationHeader)
        }
    }?;

    let header = jsonwebtoken::decode_header(token).map_err(Error::JwtError)?;
    let kid = header.kid.ok_or(Error::NoKid)?;
    let jwk = fetch_jwks(kid, env).await?;
    let key = jsonwebtoken::DecodingKey::from_jwk(&jwk).map_err(Error::JwtError)?;

    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
    let mut aud = HashSet::new();
    aud.insert("https://bb2b-api.atty303.workers.dev/".to_string());
    validation.aud = Some(aud);
    let data =
        jsonwebtoken::decode::<Claims>(&token, &key, &validation).map_err(Error::JwtError)?;

    Ok(data)
}

async fn fetch_jwks(kid: String, env: worker::Env) -> Result<jsonwebtoken::jwk::Jwk, Error> {
    let url = "https://bb2b.us.auth0.com/.well-known/jwks.json";
    let kv = env.kv("JWKS").map_err(Error::WorkerError)?;
    let name = format!("{}:{}", url, kid);
    let cached = kv
        .get(&name)
        .json::<jsonwebtoken::jwk::Jwk>()
        .await
        .map_err(Error::KvError);
    let cached = if let Ok(cached) = cached {
        Ok(cached)
    } else {
        Ok(None)
    }?;
    if let Some(cached) = cached {
        Ok(cached)
    } else {
        console_debug!("fetching jwk: {}", name);
        let jwks = reqwest::get(url)
            .await
            .map_err(Error::FetchError)?
            .json::<jsonwebtoken::jwk::JwkSet>()
            .await
            .map_err(Error::FetchError)?;
        console_debug!("fetched jwks: {:?}", jwks);
        let jwk = jwks.find(&kid).cloned().ok_or(Error::JwkNotFound(kid))?;
        console_debug!("caching jwk: {}", name);
        kv.put(&name, &jwk)
            .map_err(Error::KvError)?
            .execute()
            .await
            .map_err(Error::KvError)?;
        console_debug!("cached jwk: {}", name);
        Ok(jwk)
    }
}
