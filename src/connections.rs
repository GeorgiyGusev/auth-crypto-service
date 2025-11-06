use std::sync::Arc;

use vaultrs::client::VaultClient;

pub struct Connections {
    pub vault_conn: Arc<VaultClient>,
}
