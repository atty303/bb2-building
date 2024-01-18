use std::rc::Rc;

use indicium::simple::{Indexable, KString, SearchIndexBuilder, Tokenizer};

use data::skill::Skill;
use data::Database;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/tokenizer.js")]
extern "C" {
    fn tokenize(string: String) -> Vec<String>;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct Document {
    pub skill: Skill,
}

impl Indexable for Document {
    fn strings(&self) -> Vec<String> {
        self.skill
            .modes
            .iter()
            .map(|mode| format!("{}", mode.format()))
            .collect()
    }
}

pub fn create(db: &Database) {
    let tokenizer: Tokenizer = Rc::new(Box::new(|string| {
        let mut tokens = vec![];
        tokenize(string.to_string())
            .into_iter()
            .map(|s| {
                tokenize(s)
                    .into_iter()
                    .map(|s| KString::from_string(s))
                    .collect::<Vec<_>>()
            })
            .for_each(|mut v| tokens.append(&mut v));
        tokens
    }));

    let mut index = SearchIndexBuilder::default()
        .tokenizer(Some(tokenizer))
        .build();
    for skill in db.skill.values() {
        index.insert(
            &skill.hash,
            &Document {
                skill: skill.clone(),
            },
        );
    }

    let r = index.search("猛毒");

    log(&format!("{:?}", r));
}
