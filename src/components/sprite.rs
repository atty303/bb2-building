use dioxus::prelude::*;

#[component]
pub fn Sprite<'a>(cx: Scope<'a>, sprite: &'a data::Sprite) -> Element {
    render! {
        span {
            class: "inline-block sprite",
            style: "--x: {sprite.x}px; --y: {sprite.y}px; --w: {sprite.width}px; --h: {sprite.height}px; --s: 0.5"
        }
    }

}
