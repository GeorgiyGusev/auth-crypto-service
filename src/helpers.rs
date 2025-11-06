use anyhow::{Context, Result};
use std::sync::Arc;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

pub fn init_vault_client() -> Result<Arc<VaultClient>> {
    let vault_settings = VaultClientSettingsBuilder::default()
        .build()
        .context("Неудалось получить env для конфига или ошибка при билде")?;
    let vault_conn = VaultClient::new(vault_settings).context("Не удалось создать клиент")?;
    Ok(Arc::new(vault_conn))
}
