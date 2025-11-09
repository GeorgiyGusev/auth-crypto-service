use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::keys::{
    models::{CreateJwtRequest, SignJwtTokenResponse},
    service::jwt_service::JwtService,
    state::KeysState,
};

pub async fn sign_jwt_token(
    State(state): State<Arc<KeysState>>,
    Json(payload): Json<CreateJwtRequest>,
) -> Result<Json<SignJwtTokenResponse>, (StatusCode, String)> {
    let key_entry = state.keys_store.get_actual_private_key().await.ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        "No keys available".to_string(),
    ))?;

    let token = JwtService::sign_jwt(payload.payload, &key_entry)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(SignJwtTokenResponse { token }))
}
