use dioxus::prelude::*;
use dioxus_signals::*;
use fermi::{use_atom_state, use_read};

use data::skill::{Skill, SkillHash};

use crate::atoms::{DATABASE, SEARCH_CATALOGS};

#[derive(PartialEq, Clone)]
pub struct UseSearchSkill {
    pub query: Signal<String>,
    pub results: Signal<Vec<Signal<Skill>>>,
}

pub fn use_search_skill(cx: &ScopeState) -> &UseSearchSkill {
    let db = use_read(cx, &DATABASE);
    let catalog = use_atom_state(cx, &SEARCH_CATALOGS);

    let query = use_signal(cx, || String::new());
    let results = use_signal(cx, || Vec::<Signal<Skill>>::new());

    use_effect_with_dependencies(cx, (db, catalog), move |(db, catalog)| {
        let hashes: Vec<SkillHash> = if query.read().is_empty() {
            db.skill.iter().map(|skill| skill.hash).collect()
        } else {
            catalog
                .skill
                .search(&query.read())
                .iter()
                .map(|hash| **hash)
                .collect()
        };
        let items = hashes
            .iter()
            .map(|hash| Signal::new(db.skill.get(hash).unwrap().clone()))
            .collect();
        results.set(items);
    });

    cx.use_hook(|| UseSearchSkill { query, results })
}
