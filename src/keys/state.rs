use crate::keys::repo::keys_repo_impl::VaultKeysRepo;

use super::service::key_store::KeyStore;

pub struct KeysState {
    pub keys_store: KeyStore<VaultKeysRepo>,
}
