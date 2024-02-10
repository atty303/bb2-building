use worker::event;

#[event(fetch)]
pub async fn main(
    req: worker::Request,
    env: worker::Env,
    ctx: worker::Context,
) -> worker::Result<worker::Response> {
    let mut res = worker::Response::from_body(worker::ResponseBody::Body("Hello, world2!".into()));
    res
}
