use crate::components::App;

mod atoms;
mod components;
mod hooks;
mod pages;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}
