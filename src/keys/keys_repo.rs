use anyhow::Result;
use async_trait::async_trait;

use std::sync::Arc;

pub type KeysRepoAsync = Arc<dyn KeysRepo>;
#[async_trait]
pub(crate) trait KeysRepo: Send + Sync {
    async fn get_secret(&self, version: u64) -> Result<String>;
    async fn get_current_version(&self) -> Result<u64>;
    // TODO: create rotation keys
    // async fn create_new_key_version(&self, new_key: String) -> Result<()>;
}
