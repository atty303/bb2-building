use dioxus::prelude::*;
use fermi::{use_atom_state, use_read};

use data::skill::SkillHash;

use crate::atoms::{DATABASE, SEARCH_CATALOGS};

pub struct UseSearchSkill {
    pub query: UseRef<String>,
    pub results: UseRef<Vec<SkillHash>>,
}

pub fn use_search_skill(cx: &ScopeState) -> &UseSearchSkill {
    let db = use_read(cx, &DATABASE);
    let index = use_atom_state(cx, &SEARCH_CATALOGS);

    let query = use_ref(cx, || String::new());
    let results = use_ref(cx, || Vec::<SkillHash>::new());

    use_effect(cx, (db, query), move |(db, query)| {
        to_owned![index, results];
        async move {
            let q = query.read().clone();
            let indexes = if q.is_empty() {
                db.skill.iter().map(|skill| skill.hash).collect()
            } else {
                index.skill.search(&*q).iter().map(|hash| **hash).collect()
            };
            results.set(indexes);
        }
    });

    cx.use_hook(|| UseSearchSkill {
        query: query.clone(),
        results: results.clone(),
    })
}
