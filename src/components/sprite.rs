use dioxus::prelude::*;

#[component]
pub fn Sprite<'a>(cx: Scope<'a>, sprite: &'a data::Sprite) -> Element {
    let texture_height = 4096;
    let x = sprite.x;
    let y = texture_height - sprite.y - sprite.height as u16;
    let width = sprite.width;
    let height = sprite.height;
    render! {
        span {
            class: "inline-block",
            width: "{width}px",
            min_width: "{width}px",
            height: "{height}px",
            min_height: "{height}px",
            background: "url('image/a50549b8827f09843841d13f031f165f.png') -{x}px -{y}px",
        }
    }
}
