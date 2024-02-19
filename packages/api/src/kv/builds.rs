use uuid::Uuid;
use worker::kv::KvStore;
use worker::*;

use crate::types::{BuildMetadata, BuildValue};

pub struct Builds(KvStore);

impl Builds {
    pub fn new(store: KvStore) -> Self {
        Self(store)
    }

    pub async fn list_user_build(&self, user_id: String) -> Result<Vec<(Uuid, BuildMetadata)>> {
        let r = self
            .0
            .list()
            .prefix(format!("user_builds:user_id={}:id=", user_id))
            .execute()
            .await?;
        r.keys
            .iter()
            .map(|k| {
                let id = k.name.split('=').last().unwrap();
                let uuid = Uuid::parse_str(id).map_err(|e| Error::from(e.to_string()))?;
                let metadata = k.metadata.as_ref().unwrap();
                let metadata = serde_json::from_value::<BuildMetadata>(metadata.clone())?;
                Ok((uuid, metadata))
            })
            .collect()
    }

    pub async fn put_user_build(
        &self,
        user_id: String,
        value: BuildValue,
        metadata: BuildMetadata,
    ) -> Result<Uuid> {
        let uuid = Uuid::now_v7();
        self.0
            .put(
                &format!("user_builds:user_id={}:id={}", user_id, uuid),
                &value,
            )?
            .metadata(&metadata)?
            .execute()
            .await?;
        Ok(uuid)
    }
}
