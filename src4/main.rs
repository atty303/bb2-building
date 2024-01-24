use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeWebConsoleWriter};

use crate::components::App;

mod atoms;
mod components;
mod hooks;
mod pages;
mod search;

fn main() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .without_time()
        .with_writer(MakeWebConsoleWriter::new())
        .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG);
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();

    dioxus_web::launch(App);
}
