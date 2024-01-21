// TODO: move to gen
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use token::{Token, Tokens};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Term {
    pub tokens: Tokens,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TermRepository {
    inner: HashMap<String, Term>,
}

impl<'a> TermRepository {
    pub fn from_vec(vec: Vec<(String, Term)>) -> Self {
        let mut inner = HashMap::new();
        for term in vec {
            inner.insert(term.0, term.1);
        }
        Self { inner }
    }

    pub fn get_inner(&'a self, key: &str, tips: Option<String>) -> Tokens {
        let mut out = Tokens::from_vec(vec![Token::TermStart(key.to_string(), tips)]);
        match self.inner.get(key) {
            Some(v) => v.tokens.clone(),
            None => Tokens::from_vec(vec![Token::Error(key.to_string())]),
        }
        .write(&mut out);
        Token::TermEnd.write(&mut out);
        out
    }

    pub fn get(&'a self, key: &str) -> Tokens {
        self.get_inner(key, None)
    }

    pub fn get_tips(&'a self, key: &str, tips: &str) -> Tokens {
        self.get_inner(key, Some(tips.to_string()))
    }

    pub fn try_get(&'a self, key: &str) -> Option<Tokens> {
        match self.inner.get(key) {
            Some(v) => Some(v.tokens.clone()),
            None => None,
        }
    }

    pub fn get_str(&self, key: &str) -> String {
        format!("{}", self.get(key))
    }
}

impl Deref for TermRepository {
    type Target = HashMap<String, Term>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TermRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
