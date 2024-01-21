use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::rc::Rc;

use indicium::simple::{Indexable, KString, SearchIndex, SearchIndexBuilder, Tokenizer};
use ref_cast::RefCast;
use wasm_bindgen::prelude::*;

use data::skill::{Skill, SkillHash, SkillRepository};
use data::{
    Repository, Rune, RuneHash, RuneRepository, Search, SearchIndexable, SearchMarker,
    ToSearchMaker, LANGUAGES,
};

use std::hash::Hasher;

pub struct SearchCatalog<M: SearchMarker, T: Search<M>, R: Repository<T::Key, T::Item> + Default> {
    pub index: SearchIndex<T::Key>,
    pub repository: Rc<R>,
}

impl<M: SearchMarker, T: Search<M>, R: Repository<T::Key, T::Item> + Default> PartialEq
    for SearchCatalog<M, T, R>
{
    fn eq(&self, other: &Self) -> bool {
        let mut hasher_self = DefaultHasher::new();
        self.index.hash(&mut hasher_self);

        let mut hasher_other = DefaultHasher::new();
        other.index.hash(&mut hasher_other);

        hasher_self.finish() == hasher_other.finish()
    }
}

impl<M: SearchMarker, T: Search<M>, R: Repository<T::Key, T::Item> + Default>
    SearchCatalog<M, T, R>
{
    pub fn search<'a>(&'a self, query: &'a str) -> Vec<&'a T::Key> {
        self.index.search(query)
    }
}

impl<M: SearchMarker, T: Search<M>, R: Repository<T::Key, T::Item> + Default> Default
    for SearchCatalog<M, T, R>
{
    fn default() -> Self {
        Self {
            index: SearchIndex::default(),
            repository: R::default().into(),
        }
    }
}

#[derive(RefCast)]
#[repr(transparent)]
pub struct SkillSearch(Skill);

impl SearchMarker for SkillSearch {}

impl<'a> Search<SkillSearch> for SkillSearch {
    type Key = SkillHash;
    type Item = Skill;
    type Repository = SkillRepository;
    type Marker = SkillSearch;
}

impl ToSearchMaker<SkillSearch, SkillSearch> for SkillSearch {
    fn to_search_marker(item: &Skill) -> &SkillSearch {
        SkillSearch::ref_cast(item)
    }
}

impl<'a> Indexable for SkillSearch {
    fn strings(&self) -> Vec<String> {
        <Skill as SearchIndexable<SkillHash, SkillSearch, SkillSearch>>::strings(&self.0)
    }
}

#[derive(RefCast)]
#[repr(transparent)]
pub struct RuneSearch(Rune);

impl SearchMarker for RuneSearch {}

impl<'a> Search<RuneSearch> for RuneSearch {
    type Key = RuneHash;
    type Item = Rune;
    type Repository = RuneRepository;
    type Marker = RuneSearch;
}

impl ToSearchMaker<RuneSearch, RuneSearch> for RuneSearch {
    fn to_search_marker(item: &Rune) -> &RuneSearch {
        RuneSearch::ref_cast(item)
    }
}

impl<'a> Indexable for RuneSearch {
    fn strings(&self) -> Vec<String> {
        <Rune as SearchIndexable<RuneHash, RuneSearch, RuneSearch>>::strings(&self.0)
    }
}

#[derive(Default, PartialEq)]
pub struct SearchCatalogs {
    pub skill: SearchCatalog<SkillSearch, SkillSearch, SkillRepository>,
    pub rune: SearchCatalog<RuneSearch, RuneSearch, RuneRepository>,
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

pub fn create_catalog<'a, M: SearchMarker + Indexable, T: Search<M>, N: ToSearchMaker<M, T>>(
    repository: Rc<T::Repository>,
    language: String,
) -> SearchCatalog<M, T, T::Repository>
where
    <T as Search<M>>::Repository: Default,
{
    let lang = LANGUAGES.iter().find(|l| *l == &language).unwrap();

    let tokenizer: Tokenizer = Rc::new(Box::new(|string| {
        let intl = IntlTokenizer::new(lang.to_string());
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
        index.insert(id, N::to_search_marker(item));
    }

    SearchCatalog {
        index,
        repository: repository,
    }
}
