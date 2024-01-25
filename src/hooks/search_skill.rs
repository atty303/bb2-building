use dioxus::prelude::*;

use crate::global::{DATABASE, SEARCH_CATALOGS};
use data::skill::{Skill, SkillHash};

#[derive(PartialEq, Clone)]
pub struct UseSearchSkill {
    pub query: Signal<String>,
    pub results: Signal<Vec<Signal<Skill>>>,
}

pub fn use_search_skill() -> UseSearchSkill {
    let query = use_signal(|| String::new());
    let results = use_signal(|| Vec::<Signal<Skill>>::new());

    use_effect(move || {
        let hashes: Vec<SkillHash> = if query().is_empty() {
            DATABASE().skill.iter().map(|skill| skill.hash).collect()
        } else {
            SEARCH_CATALOGS
                .read()
                .skill
                .search(&query.read())
                .iter()
                .map(|hash| **hash)
                .collect()
        };
        let items = hashes
            .iter()
            .map(|hash| Signal::new(DATABASE().skill.get(hash).unwrap().clone()))
            .collect();
        *results.write() = items;
    });

    use_hook(|| UseSearchSkill { query, results })
}
