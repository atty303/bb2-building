#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Sprite<'a>(cx: Scope<'a>, sprite: &'a data::Sprite) -> Element {
    let x = sprite.x;
    let y = 4096 - sprite.y - sprite.height as u16;
    render! {
        span {
            style:
                r#"
                    display: inline-block;
                    width: {sprite.width}px;
                    height: {sprite.height}px;
                    background: url('image/a50549b8827f09843841d13f031f165f.png') -{x}px -{y}px;
                "#
        }
    }
}
