use std::collections::HashMap;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};

use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};
use token::{Token, Tokens};

#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    pub tokens: Tokens,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
struct TermSer {
    key: String,
    tokens: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct TermRepository {
    inner: HashMap<String, Term>,
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

impl<'a> TermRepository {
    pub fn write<'i, W: Write, I: Iterator<Item = &'i (String, Term)>>(
        avro_write: W,
        terms: I,
    ) -> Result<(), apache_avro::Error> {
        let schema = TermSer::get_schema();
        let mut writer =
            apache_avro::Writer::with_codec(&schema, avro_write, apache_avro::Codec::Deflate);
        for (key, term) in terms {
            let mut out = vec![];
            for n in &term.tokens.0 {
                match n {
                    Token::Text(s) => out.push(format!(" {}", s)),
                    Token::Var(s) => out.push(format!("${}", s)),
                    Token::NewLine => out.push("~".to_string()),
                    Token::Empty => (),
                    Token::Error(s) => out.push(format!("!{}", s)),
                }
            }
            writer.append_ser(&TermSer {
                key: key.to_string(),
                tokens: out,
            })?;
        }
        Ok(())
    }

    pub fn read<R: Read>(avro_read: R) -> Result<Self, apache_avro::Error> {
        let schema = TermSer::get_schema();
        let reader = apache_avro::Reader::with_schema(&schema, avro_read)?;
        let mut map = HashMap::new();
        for result in reader {
            let value = &result?;
            let r = apache_avro::from_value::<TermSer>(&value)?;

            let nodes = r
                .tokens
                .iter()
                .map(|s| {
                    if s.starts_with(" ") {
                        Token::Text(s[1..].to_string())
                    } else if s.starts_with("$") {
                        Token::Var(s[1..].to_string())
                    } else if s.starts_with("!") {
                        Token::Error(s[1..].to_string())
                    } else if s == "~" {
                        Token::NewLine
                    } else {
                        panic!("invalid term: {}", s);
                    }
                })
                .collect::<Vec<_>>();

            map.insert(
                r.key,
                Term {
                    tokens: Tokens(nodes),
                },
            );
        }
        Ok(TermRepository { inner: map })
    }

    pub fn get(&'a self, key: &str) -> Tokens {
        match self.inner.get(key) {
            Some(v) => v.tokens.clone(),
            None => Tokens(vec![Token::Error(key.to_string())]),
        }
    }

    pub fn tr<T, F: Fn(&Tokens) -> T>(&'a self, key: &str, f: F) -> T {
        match self.inner.get(key) {
            Some(v) => f(&v.tokens),
            None => f(&Tokens(vec![Token::Error(key.to_string())])),
        }
    }
}
