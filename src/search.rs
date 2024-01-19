use std::rc::Rc;

use indicium::simple::{Indexable, KString, SearchIndex, SearchIndexBuilder, Tokenizer};
use wasm_bindgen::prelude::*;

use data::skill::{Skill, SkillHash, SkillRepository};
use data::{Database, Repository, Search, SearchIndexable, SearchMarker};

pub struct SearchCatalog<'a, M: SearchMarker, T: Search<M>, R: Repository<T::Key, T::Item>> {
    pub index: SearchIndex<T::Key>,
    pub repository: &'a R,
}

pub struct SkillSearch(Skill);

impl SearchMarker for SkillSearch {
    fn new(&self, item: Self::Item) -> Self {
        Self(item)
    }
}

impl Search<SkillSearch> for SkillSearch {
    type Key = SkillHash;
    type Item = Skill;
    type Repository = SkillRepository;
    type Marker = SkillSearch;
}

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

impl Indexable for SkillSearch {
    fn strings(&self) -> Vec<String> {
        todo!("SkillSearch::strings")
        //<SkillSearch as SearchMarker>::T::Item::strings(self)
    }
}

// impl<K: Ord + SearchIndexable<K>> Indexable for SearchIndexable<K> {
//     fn strings(&self) -> Vec<String> {
//         self.strings()
//     }
// }

pub fn create_catalog<M: SearchMarker + Indexable, T: Search<M>>(
    repository: &T::Repository,
) -> SearchCatalog<M, T, T::Repository> {
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
    for id in repository.iter() {
        let item = repository.get(id).unwrap();
        index.insert(id, &item.lift());
    }

    SearchCatalog { index, repository }
}
