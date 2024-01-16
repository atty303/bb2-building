use std::collections::HashMap;
use std::io::Read;
use std::ops::Deref;

use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AvroSchema)]
pub struct State {
    pub row_id: String,
    pub id: String,
    pub format: String,
    pub long_format: String,
}

#[derive(Clone, Default)]
pub struct StateRepository {
    inner: HashMap<String, State>,
}

impl StateRepository {
    pub fn read<R: Read>(avro_read: R) -> Result<StateRepository, apache_avro::Error> {
        let mut out = HashMap::new();
        let schema = State::get_schema();
        let reader = apache_avro::Reader::with_schema(&schema, avro_read)?;
        for result in reader {
            let value = result?;
            let r = apache_avro::from_value::<State>(&value)?;
            out.insert(r.row_id.clone(), r);
        }
        Ok(StateRepository { inner: out })
    }
}

impl Deref for StateRepository {
    type Target = HashMap<String, State>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
