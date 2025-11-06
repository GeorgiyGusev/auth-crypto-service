// keys_repo_impl.rs (реализация для Vault)
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use vaultrs::{client::VaultClient, kv2};

use crate::keys::keys_repo::KeysRepo;

#[derive(Debug, Deserialize, Serialize)]
struct Key {
    key: String,
}

const CRYPTO_KEY_PATH: &str = "crypto-key";
const VAULT_STORE: &str = "secret";

pub struct VaultKeysRepo {
    client: Arc<VaultClient>,
}

impl VaultKeysRepo {
    pub fn new(client: Arc<VaultClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl KeysRepo for VaultKeysRepo {
    async fn get_secret(&self, version: u64) -> Result<String> {
        let res: Key =
            kv2::read_version(self.client.as_ref(), VAULT_STORE, CRYPTO_KEY_PATH, version)
                .await
                .with_context(|| {
                    format!("failed to read vault kv2 version {version} at {CRYPTO_KEY_PATH}")
                })?;
        Ok(res.key)
    }

    async fn get_current_version(&self) -> Result<u64> {
        let meta = kv2::read_metadata(self.client.as_ref(), VAULT_STORE, CRYPTO_KEY_PATH)
            .await
            .context("failed to read kv2 metadata")?;
        Ok(meta.current_version)
    }

    // TODO: create rotation keys
    // async fn create_new_key_version(&self, new_key: String) -> Result<()> {
    //     kv2::set(
    //         self.client.as_ref(),
    //         VAULT_STORE,
    //         CRYPTO_KEY_PATH,
    //         &Key { key: new_key },
    //     )
    //     .await
    //     .context("failed to write new crypto key")?;
    //     Ok(())
    // }
}
