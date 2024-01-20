use dioxus::prelude::*;
use dioxus_signals::*;
use fermi::{use_atom_state, use_read};

use data::skill::{Skill, SkillHash};

use crate::atoms::{DATABASE, SEARCH_CATALOGS};

pub struct UseSearchSkill {
    pub query: Signal<String>,
    pub results: Signal<Vec<Signal<Skill>>>,
}

pub fn use_search_skill(cx: &ScopeState) -> &UseSearchSkill {
    let db = use_read(cx, &DATABASE);
    let index = use_atom_state(cx, &SEARCH_CATALOGS);

    let query = use_signal(cx, || String::new());
    let results = use_signal(cx, || Vec::<Signal<Skill>>::new());

    dioxus_signals::use_effect_with_dependencies(cx, (db, index), move |(db, index)| {
        let q = query.read().clone();
        let indexes: Vec<SkillHash> = if q.is_empty() {
            db.skill.iter().map(|skill| skill.hash).collect()
        } else {
            index.skill.search(&*q).iter().map(|hash| **hash).collect()
        };
        let items = indexes
            .iter()
            .map(|hash| Signal::new(db.skill.get(hash).unwrap().clone()))
            .collect();
        results.set(items);
    });

    cx.use_hook(|| UseSearchSkill {
        query: query.clone(),
        results: results.clone(),
    })
}
