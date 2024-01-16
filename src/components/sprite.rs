use dioxus::prelude::*;

#[component]
pub fn Sprite<'a>(cx: Scope<'a>, sprite: &'a data::Sprite, scale: f64) -> Element {
    render! {
        span {
            class: "inline-block sprite",
            style: "--x: {sprite.x}px; --y: {sprite.y}px; --w: {sprite.width}px; --h: {sprite.height}px; --s: {scale}"
        }
    }
}

#[component]
pub fn SpriteIcon<'a>(
    cx: Scope<'a>,
    sprite: &'a data::Sprite,
    size: i32,
    class: &'a str,
) -> Element {
    let sprite_size = sprite.width.max(sprite.height);
    let scale = *size as f64 / sprite_size as f64;
    render! {
        span { class: "inline-block align-middle bg-black overflow-hidden {class}",
            width: "{size}px",
            height: "{size}px",
            line_height: "{size}px",
            Sprite { sprite: sprite, scale: scale }
        }
    }
}
