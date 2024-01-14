use std::collections::HashMap;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct Term {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AvroSchema)]
struct TermSer {
    key: String,
    term: Term,
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

impl TermMap {
    pub fn new() -> Self {
        let map = HashMap::new();
        Self { inner: map }
    }

    pub fn write<W: Write>(avro_write: W, term_map: &Self) -> Result<(), apache_avro::Error> {
        let schema = TermSer::get_schema();
        let mut writer = apache_avro::Writer::new(&schema, avro_write);
        for (key, term) in term_map.inner.iter() {
            writer.append_ser(TermSer { key: key.clone(), term: term.clone() })?;
        }
        Ok(())
    }

    pub fn read<R: Read>(avro_read: R) -> Result<Self, apache_avro::Error> {
        let reader = apache_avro::Reader::new(avro_read)?;
        let mut map = HashMap::new();
        for result in reader {
            let value = &result.expect("Error reading value from avro reader");
            let r = apache_avro::from_value::<TermSer>(&value).expect("Error deserializing value");
            map.insert(r.key, r.term);
        }
        Ok(TermMap { inner: map })
    }

    pub fn tr<'a>(&'a self, key: &'a Tr) -> String {
        let key = match key {
            Tr::Name(id) => format!("NM-{}", id),
            Tr::Action(action_type) => format!("DC-SkillNodeDesc-{}", action_type),
        };
        self.inner.get(&key).map(|v| &v.value).unwrap_or(&key).to_string()
    }
}

pub enum Tr<'a> {
    Name(&'a str),
    Action(&'a str),
}