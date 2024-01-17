use std::collections::HashMap;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub row_id: String,
    pub id: String,
    pub format: String,
    pub long_format: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct StateRepository {
    inner: HashMap<String, State>,
}

impl StateRepository {
    pub fn from_vec(vec: Vec<State>) -> Self {
        let mut inner = HashMap::new();
        for state in vec {
            inner.insert(state.row_id.clone(), state);
        }
        Self { inner }
    }
}

impl Deref for StateRepository {
    type Target = HashMap<String, State>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
