use dioxus::prelude::*;
use dioxus_signals::*;
use fermi::{use_atom_state, use_read};

use data::skill::SkillHash;
use data::Rune;

use crate::atoms::{DATABASE, SEARCH_CATALOGS};
use data::Repository;

#[derive(PartialEq, Clone)]
pub struct UseSearchRune {
    pub query: Signal<String>,
    pub results: Signal<Vec<Signal<Rune>>>,
}

pub fn use_search_rune(cx: &ScopeState) -> &UseSearchRune {
    let db = use_read(cx, &DATABASE);
    let catalog = use_atom_state(cx, &SEARCH_CATALOGS);

    let query = use_signal(cx, || String::new());
    let results = use_signal(cx, || Vec::<Signal<Rune>>::new());

    use_effect_with_dependencies(cx, (db, catalog), move |(db, catalog)| {
        let hashes: Vec<SkillHash> = if query.read().is_empty() {
            db.rune.iter().map(|rune| rune.hash).collect()
        } else {
            catalog
                .rune
                .search(&query.read())
                .iter()
                .map(|hash| **hash)
                .collect()
        };
        let items = hashes
            .iter()
            .map(|hash| Signal::new(db.rune.get(hash).unwrap().clone()))
            .collect();
        results.set(items);
    });

    cx.use_hook(|| UseSearchRune { query, results })
}
