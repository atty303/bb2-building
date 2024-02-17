// use app::app::App;
// use dioxus::prelude::*;
// use dioxus_fullstack::prelude::*;
// use worker::event;
//
// #[event(fetch)]
// pub async fn main(
//     req: worker::Request,
//     env: worker::Env,
//     ctx: worker::Context,
// ) -> worker::Result<worker::Response> {
//     let handler = serve_dioxus_application("");
//     let rep = handler(req, env);
//     rep.await
// }
