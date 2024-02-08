use dioxus::prelude::*;
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
use tracing_web::{performance_layer, MakeWebConsoleWriter};

use crate::app::App;

mod app;
mod components;
mod editor;
mod global;
mod hooks;
mod pages;
mod search;
mod ui;

fn main() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .without_time()
        .with_level(false)
        .with_writer(MakeWebConsoleWriter::new().with_pretty_level())
        .with_filter(EnvFilter::new("web=debug"));
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();

    launch(App);
}
