use crate::components::app::App;

mod components;
mod hooks;
mod pages;
mod atoms;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}
