use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GlobalRepository {
    pub rarity_colors: Vec<String>,
}
