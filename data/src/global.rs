use serde::{Deserialize, Serialize};

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GlobalRepository {
    pub rarity_colors: Vec<String>,
}
