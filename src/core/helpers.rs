use anyhow::{Context, Result};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

pub fn init_vault_client() -> Result<VaultClient> {
    let vault_settings = VaultClientSettingsBuilder::default()
        .build()
        .context("Неудалось получить env для конфига или ошибка при билде")?;
    let vault_client = VaultClient::new(vault_settings).context("Не удалось создать клиент")?;
    Ok(vault_client)
}
