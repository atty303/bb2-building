use dioxus::prelude::*;
use fermi::use_read;

use crate::atoms::DATABASE;
use crate::components::Icon;

#[component]
pub fn Rarity(cx: Scope, rarity: i8) -> Element {
    let db = use_read(cx, &DATABASE);
    let default = "000000".to_string();
    let color = db
        .global
        .rarity_colors
        .get(*rarity as usize - 1)
        .unwrap_or(&default);
    //format!("{}", db.term().get(&format!("CLR-Star-Rarity-{}", rarity)));
    render! {
        span {
            class: "bg-neutral m-2 px-2 rounded",
            color: "#{color}",
            for _ in 0..*rarity {
                Icon { class: "inline-block w-4 h-4",
                    svg: r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-4 h-4"><path fill-rule="evenodd" d="M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.006 5.404.434c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.434 2.082-5.005Z" clip-rule="evenodd" /></svg>"#
                }
            }
        }
    }
}
