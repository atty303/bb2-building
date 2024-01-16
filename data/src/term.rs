use std::collections::HashMap;
use std::fmt::Arguments;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use token::{Token, Tokens};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Term {
    pub tokens: Tokens,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

    pub fn read<R: Read>(read: R) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::decode::from_read(read)
    }

    pub fn write<W: Write>(&self, write: &mut W) -> Result<(), rmp_serde::encode::Error> {
        rmp_serde::encode::write(write, self)
    }

    pub fn get(&'a self, key: &str) -> Tokens {
        match self.inner.get(key) {
            Some(v) => v.tokens.clone(),
            None => Tokens(vec![Token::Error(key.to_string())]),
        }
    }

    pub fn get_fmt(&self, args: &Arguments<'_>) -> Tokens {
        if let Some(key) = args.as_str() {
            self.get(key)
        } else {
            Tokens(vec![Token::Error(format!("{:?}", args))])
        }
    }

    pub fn get_fmt_str(&self, args: &Arguments<'_>) -> String {
        format!("{}", self.get_fmt(args))
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
