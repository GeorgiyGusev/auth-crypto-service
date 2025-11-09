use super::{repo::keys_repo_impl::VaultKeysRepo, service::key_store::KeyStore};
use anyhow::Result;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use super::handlers::{jwks, refresh_keys, sign_jwt_token};

pub async fn init_keys_router(vault_client: vaultrs::client::VaultClient) -> Result<Router> {
    let repo_impl = VaultKeysRepo::new(vault_client);
    let key_store = KeyStore::new(repo_impl);
    key_store.load_keys().await?;

    let state = Arc::new(super::state::KeysState {
        keys_store: key_store,
    });

    let router = Router::new()
        .route("/.well-known/jwks.json", get(jwks))
        .route("/sign/jwt", post(sign_jwt_token))
        .route("/refresh/keys", post(refresh_keys))
        .with_state(state);

    Ok(router)
}
