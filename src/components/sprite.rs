use dioxus::prelude::*;
use dioxus_signals::ReadOnlySignal;

#[component]
pub fn Sprite(cx: Scope, sprite: ReadOnlySignal<data::Sprite>, scale: f64) -> Element {
    render! {
        span {
            class: "inline-block sprite",
            style: "--x: {sprite.read().x}px; --y: {sprite.read().y}px; --w: {sprite.read().width}px; --h: {sprite.read().height}px; --s: {scale}"
        }
    }
}

#[component]
pub fn SpriteIcon<'a>(
    cx: Scope<'a>,
    sprite: ReadOnlySignal<data::Sprite>,
    size: i32,
    class: &'a str,
) -> Element {
    let sprite_size = sprite.read().width.max(sprite.read().height);
    let scale = *size as f64 / sprite_size as f64;
    render! {
        span { class: "inline-block align-middle bg-black overflow-hidden {class}",
            width: "{size}px",
            height: "{size}px",
            line_height: "{size}px",
            Sprite { sprite: *sprite, scale: scale }
        }
    }
}
