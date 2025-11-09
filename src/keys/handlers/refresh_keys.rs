use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::keys::models::RefreshKeysResponse;
use crate::keys::state::KeysState;

pub async fn refresh_keys(
    State(state): State<Arc<KeysState>>,
) -> Result<Json<RefreshKeysResponse>, (StatusCode, String)> {
    state
        .keys_store
        .load_keys()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(RefreshKeysResponse {
        status: "ok".to_string(),
    }))
}
