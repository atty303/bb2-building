use dioxus::prelude::*;

#[component]
pub fn Sprite<'a>(cx: Scope<'a>, sprite: &'a data::Sprite, scale: f64) -> Element {
    render! {
        span {
            class: "inline-block sprite rounded bg-black",
            style: "--x: {sprite.x}px; --y: {sprite.y}px; --w: {sprite.width}px; --h: {sprite.height}px; --s: {scale}"
        }
    }
}
