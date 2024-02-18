mod jwt;

use worker::*;

#[event(start)]
fn start() {
    console_error_panic_hook::set_once();
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let token = jwt::verify_jwt(req, env)
        .await
        .map_err(|e| Error::RustError(e.to_string()))?;
    console_log!("token: {:?}", token);

    Response::ok("OK")

    // let router = Router::new();
    // router
    //     .get_async("/build/:lang", |req, ctx| async move {
    //         if let Some(lang) = ctx.param("lang") {
    //             let builds = ctx.kv("BUILDS")?;
    //             let result = builds.list().execute().await?;
    //             result.keys
    //             Response::error("Bad Request", 400)
    //         } else {
    //             Response::error("Bad Request", 400)
    //         }
    //     })
    //     .run(req, env)
    //     .await
}
