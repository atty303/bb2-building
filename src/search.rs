use std::rc::Rc;

use indicium::simple::{Indexable, KString, SearchIndex, SearchIndexBuilder, Tokenizer};
use wasm_bindgen::prelude::*;

use data::skill::{Skill, SkillHash, SkillRepository};
use data::{Database, Repository, Search, SearchIndexable, SearchMarker};

pub struct SearchCatalog<
    'a,
    M: SearchMarker,
    T: Search<M>,
    R: Repository<T::Key, T::Item> + Default,
> {
    pub index: SearchIndex<T::Key>,
    pub repository: &'a R,
}

impl<'a, M: SearchMarker, T: Search<M>, R: Repository<T::Key, T::Item> + Default>
    SearchCatalog<'a, M, T, R>
{
    pub fn search(&self, query: &str) -> Vec<&'a T::Key> {
        self.index.search(query)
    }
}

impl<'a, M: SearchMarker, T: Search<M>, R: Repository<T::Key, T::Item> + Default> Default
    for SearchCatalog<'a, M, T, R>
{
    fn default() -> Self {
        Self {
            index: SearchIndex::default(),
            repository: &R::default(),
        }
    }
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
pub struct SearchCatalogs<'a> {
    pub skill: SearchCatalog<'a, SkillSearch, SkillSearch, SkillRepository>,
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
) -> SearchCatalog<M, T, T::Repository>
where
    <T as Search<M>>::Repository: Default,
{
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
