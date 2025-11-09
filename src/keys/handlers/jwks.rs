use std::sync::Arc;

use crate::keys::{models, state::KeysState};
use axum::extract::{Json, State};

pub async fn jwks(State(state): State<Arc<KeysState>>) -> Json<models::JwksResponse> {
    let jwks = state.keys_store.get_jwk_list().await;
    Json(jwks)
}
