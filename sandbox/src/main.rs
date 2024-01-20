use dioxus::prelude::*;
use dioxus_signals::Signal;
use std::fmt::Display;

fn main() {
    println!("Hello, world!");
}

struct Item;

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[component]
fn App(cx: Scope) -> Element {
    let items = use_ref(cx, || Vec::<Signal<Item>>::new());

    render! {
        for i in items.read().iter() {
            div {
                Sub { item: *i }
            }
        }
    }
}

#[component]
fn Sub(cx: Scope, item: Signal<Item>) -> Element {
    render! {
        "{item}"
    }
}
