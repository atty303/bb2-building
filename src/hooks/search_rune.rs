use dioxus::prelude::*;

use data::Repository;
use data::Rune;
use data::RuneHash;

use crate::global::{DATABASE, SEARCH_CATALOGS};

#[derive(PartialEq, Clone)]
pub struct UseSearchRune {
    pub query: Signal<String>,
    pub results: Signal<Vec<Signal<Rune>>>,
}

pub fn use_search_rune() -> UseSearchRune {
    let query = use_signal(|| String::new());
    let results = use_signal(|| Vec::<Signal<Rune>>::new());

    use_effect(move || {
        let hashes: Vec<RuneHash> = if query().is_empty() {
            DATABASE().rune.iter().map(|rune| rune.hash).collect()
        } else {
            SEARCH_CATALOGS
                .read()
                .rune
                .search(&query())
                .iter()
                .map(|hash| **hash)
                .collect()
        };
        let items = hashes
            .iter()
            .map(|hash| Signal::new(DATABASE().rune.get(hash).unwrap().clone()))
            .collect();
        *results.write() = items;
    });

    use_hook(|| UseSearchRune { query, results })
}
