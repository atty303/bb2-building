use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use token::Tokens;
use Repository;
use {Search, SearchIndexable};
use {SearchMarker, Sprite};

pub type RuneHash = u16;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rune {
    pub hash: RuneHash,
    pub id: String,
    pub order: i16,
    pub icon: Sprite,
    pub rarity: i8,
    // extra fields
    pub name: String,
    pub description: Tokens,
}

impl Rune {
    pub fn format(&self) -> Tokens {
        self.description.clone()
    }
}

impl<M: SearchMarker, N: Search<M>> SearchIndexable<RuneHash, M, N> for Rune {
    fn id(&self) -> RuneHash {
        self.hash
    }

    fn strings(&self) -> Vec<String> {
        vec![self.name.clone(), format!("{}", self.format())]
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuneRepository {
    inner: HashMap<RuneHash, Rune>,
    order: Vec<RuneHash>,
}

impl RuneRepository {
    pub fn from_vec(items: Vec<Rune>) -> Self {
        let mut inner = HashMap::new();
        let mut order = vec![];
        for item in items {
            let hash = item.hash;
            inner.insert(hash, item);
            order.push(hash);
        }
        Self { inner, order }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Rune> {
        self.order.iter().map(move |k| &self.inner[k])
    }
}

impl Repository<RuneHash, Rune> for RuneRepository {
    fn get(&self, key: &RuneHash) -> Option<&Rune> {
        self.inner.get(key)
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a RuneHash> + 'a>
    where
        RuneHash: 'a,
    {
        Box::new(self.order.iter())
    }
}
