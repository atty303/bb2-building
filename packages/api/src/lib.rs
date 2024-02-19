mod jwt;
mod kv;
mod types;

use crate::jwt::Claims;
use crate::types::{
    BuildMetadata, ListViewerBuildResponse, PostViewerBuildRequest, PostViewerBuildResponse,
};
use jsonwebtoken::TokenData;
use worker::*;

#[event(start)]
fn start() {
    console_error_panic_hook::set_once();
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        // .get_async("/build/:lang", |req, ctx| async move {
        //     if let Some(lang) = ctx.param("lang") {
        //         let builds = ctx.kv("BUILDS")?;
        //         let result = builds.list().execute().await?;
        //         result.keys
        //         Response::error("Bad Request", 400)
        //     } else {
        //         Response::error("Bad Request", 400)
        //     }
        // })
        .get_async("/viewer/build", |req, ctx| async move {
            let token = require_auth(&req, &ctx).await?;
            let store = ctx.kv("BUILDS")?;
            let repo = kv::builds::Builds::new(store);
            let r = repo.list_user_build(token.claims.sub).await?;
            let items = r
                .iter()
                .map(|(i, metadata)| (i.to_string(), metadata.clone()))
                .collect::<Vec<(String, BuildMetadata)>>();
            Response::from_json(&ListViewerBuildResponse { items })
        })
        .post_async("/viewer/build", |mut req, ctx| async move {
            let token = require_auth(&req, &ctx).await?;
            let build = req.json::<PostViewerBuildRequest>().await?;
            console_debug!("req: {:?}", build);
            let store = ctx.kv("BUILDS")?;
            let repo = kv::builds::Builds::new(store);
            let r = repo
                .put_user_build(token.claims.sub, build.value, build.metadata)
                .await?;
            Response::from_json(&PostViewerBuildResponse { id: r.to_string() })
        })
        .run(req, env)
        .await
}

async fn require_auth(req: &Request, ctx: &RouteContext<()>) -> Result<TokenData<Claims>> {
    jwt::verify_jwt(req, &ctx.env)
        .await
        .map_err(|e| Error::RustError(e.to_string()))
}
