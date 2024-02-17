use dioxus::prelude::*;

#[component]
pub fn Sprite(sprite: Signal<data::Sprite>, scale: f64) -> Element {
    rsx! {
        span { class: "inline-block sprite sprite-{sprite().index}",
            style: "--x: {sprite().x}px; --y: {sprite().y}px; --w: {sprite().width}px; --h: {sprite().height}px; --s: {scale}"
        }
    }
}

#[component]
pub fn SpriteIcon(sprite: Signal<data::Sprite>, size: i32, class: &'static str) -> Element {
    let sprite_size = sprite().width.max(sprite().height);
    let scale = size as f64 / sprite_size as f64;
    rsx! {
        span { class: "inline-block bg-black overflow-hidden {class}",
            width: "{size}px",
            height: "{size}px",
            line_height: "{size}px",
            Sprite { sprite, scale }
        }
    }
}
