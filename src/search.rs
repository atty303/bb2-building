use std::rc::Rc;

use indicium::simple::{Indexable, KString, SearchIndex, SearchIndexBuilder, Tokenizer};

use data::skill::{Skill, SkillHash};
use data::Database;

use wasm_bindgen::prelude::*;

#[derive(Default)]
pub struct SearchRepository {
    pub skill: SearchIndex<SkillHash>,
}

#[wasm_bindgen(module = "/src/tokenizer.js")]
extern "C" {
    type IntlTokenizer;

    #[wasm_bindgen(constructor)]
    fn new(locale: String) -> IntlTokenizer;

    #[wasm_bindgen(method)]
    fn tokenize(this: &IntlTokenizer, string: String) -> Vec<String>;
}

struct Document {
    pub skill: Skill,
}

impl Indexable for Document {
    fn strings(&self) -> Vec<String> {
        let mut strings = vec![self.skill.name.clone()];
        strings.push(
            self.skill
                .modes
                .iter()
                .flat_map(|mode| vec![mode.name.clone(), format!("{}", mode.format())])
                .collect(),
        );
        strings
    }
}

pub fn create(db: &Database) -> SearchRepository {
    let tokenizer: Tokenizer = Rc::new(Box::new(|string| {
        let intl = IntlTokenizer::new("ja-JP".to_string());
        intl.tokenize(string.to_string())
            .into_iter()
            .map(|s| KString::from_string(s))
            .collect::<Vec<_>>()
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

    SearchRepository { skill: index }
}
