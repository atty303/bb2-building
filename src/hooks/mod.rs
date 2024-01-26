use std::future::Future;

use dioxus::prelude::{current_scope_id, use_signal, Writable};

pub use self::persistent::*;
pub use self::search_rune::*;
pub use self::search_skill::*;

mod persistent;
mod search_rune;
mod search_skill;

pub fn use_on_create<T, F>(future: impl FnOnce() -> F)
where
    T: 'static,
    F: Future<Output = T> + 'static,
{
    let needs_regen = use_signal(|| true);

    if needs_regen() {
        // We don't need regen anymore
        *needs_regen.write() = false;

        let fut = future();

        current_scope_id().unwrap().push_future(async move {
            fut.await;
        });
    }
}
