use crate::components::App;

mod atoms;
mod components;
mod hooks;
mod pages;

fn main() {
    tracing_wasm::set_as_global_default();
    dioxus_web::launch(App);
}
