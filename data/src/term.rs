use std::collections::HashMap;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};

use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Text(String),
    Var(String),
    NewLine,
}

pub fn nodes_to_string(nodes: &Vec<Node>) -> String {
    nodes.iter().map(|n| match n {
        Node::Text(s) => s.clone(),
        Node::Var(s) => format!("<{}>", s),
        Node::NewLine => "\n".to_string(),
    }).collect::<Vec<_>>().join("")
}

#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    pub nodes: Vec<Node>,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
struct TermSer {
    key: String,
    term: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct TermMap {
    inner: HashMap<String, Term>,
}

impl Deref for TermMap {
    type Target = HashMap<String, Term>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TermMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> TermMap {
    pub fn new() -> Self {
        let map = HashMap::new();
        Self { inner: map }
    }

    pub fn write<'i, W: Write, I: Iterator<Item = &'i (String, Term)>>(avro_write: W, terms: I) -> Result<(), apache_avro::Error> {
        let schema = TermSer::get_schema();
        let mut writer = apache_avro::Writer::with_codec(&schema, avro_write, apache_avro::Codec::Deflate);
        for (key, term) in terms {
            let terms = term.nodes.iter().map(|n| match n {
                Node::Text(s) => format!(" {}", s),
                Node::Var(s) => format!("${}", s),
                Node::NewLine => "~".to_string(),
            }).collect::<Vec<_>>();
            writer.append_ser(&TermSer { key: key.to_string(), term: terms })?;
        }
        Ok(())
    }

    pub fn read<R: Read>(avro_read: R) -> Result<Self, apache_avro::Error> {
        let reader = apache_avro::Reader::new(avro_read)?;
        let mut map = HashMap::new();
        for result in reader {
            let value = &result.expect("Error reading value from avro reader");
            let r = apache_avro::from_value::<TermSer>(&value).expect("Error deserializing value");

            let nodes = r.term.iter().map(|s| {
                if s.starts_with(" ") {
                    Node::Text(s[1..].to_string())
                } else if s.starts_with("$") {
                    Node::Var(s[1..].to_string())
                } else if s == "~" {
                    Node::NewLine
                } else {
                    panic!("invalid term: {}", s);
                }
            }).collect::<Vec<_>>();

            map.insert(r.key, Term { nodes });
        }
        Ok(TermMap { inner: map })
    }

    // pub fn tr(&'a self, key: &str) -> Box<Arc<Vec<Node>>> {
    //     self.inner.get(key).map(|v| Box::new(Arc::clone(&v.nodes)))
    //         .unwrap_or(Box::new(Arc::new(vec![])))
    // }

    pub fn tr<T, F: Fn(&Vec<Node>) -> T>(&'a self, key: &str, f: F) -> T {
        match self.inner.get(key) {
            Some(v) => f(&v.nodes),
            None => f(&vec![Node::Text(key.to_string())]),
        }
    }
}
