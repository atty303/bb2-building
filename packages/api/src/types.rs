use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildValue {
    pub user_id: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildMetadata {
    pub is_private: bool,
    pub title: String,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostViewerBuildRequest {
    pub value: BuildValue,
    pub metadata: BuildMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostViewerBuildResponse {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListViewerBuildResponse {
    pub items: Vec<(String, BuildMetadata)>,
}
