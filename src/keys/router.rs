use anyhow::Result;
use std::sync::Arc;
use utoipa::OpenApi;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::Json as JsonResponse,
    routing::{get, post},
};

use crate::{
    connections::Connections,
    keys::{
        jwt_service::JwtService,
        key_store::KeyStore,
        keys_repo_impl::VaultKeysRepo,
        models::{CreateJwtRequest, JwksResponse, RefreshKeysResponse, SignJwtTokenResponse},
    },
};

// ---------- OPENAPI SPECIFICATION ----------
#[derive(OpenApi)]
#[openapi(
    paths(
        jwks,
        sign_jwt_token,
        refresh_keys,
    ),
    components(
        schemas(
            JwksResponse,
            SignJwtTokenResponse,
            RefreshKeysResponse,
            CreateJwtRequest,
        )
    ),
    tags(
        (name = "keys", description = "Authentication and Cryptography Service")
    )
)]
pub struct ApiDoc;

// ---------- СОСТОЯНИЕ МОДУЛЯ KEYS ----------
struct KeysState {
    keys_store: Arc<KeyStore>,
}

// ---------- ХЕНДЛЕРЫ ----------

// JWKS endpoint
#[utoipa::path(
    get,
    path = "/.well-known/jwks.json",
    tag = "keys",
    responses(
        (status = 200, description = "JWKS retrieved successfully", body = JwksResponse),
        (status = 500, description = "Internal server error")
    )
)]
async fn jwks(State(state): State<Arc<KeysState>>) -> JsonResponse<JwksResponse> {
    let jwks = state.keys_store.get_jwk_list().await;
    JsonResponse(jwks)
}

// Sign JWT token
#[utoipa::path(
    post,
    path = "/sign/jwt",
    tag = "keys",
    request_body = CreateJwtRequest,
    responses(
        (status = 200, description = "JWT signed successfully", body = SignJwtTokenResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
async fn sign_jwt_token(
    State(state): State<Arc<KeysState>>,
    Json(payload): Json<CreateJwtRequest>,
) -> Result<JsonResponse<SignJwtTokenResponse>, (StatusCode, String)> {
    let key_entry = state.keys_store.get_actual_private_key().await.ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        "No keys available".to_string(),
    ))?;

    let token = JwtService::sign_jwt(payload.payload, &key_entry)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(JsonResponse(SignJwtTokenResponse { token }))
}

// Refresh keys
#[utoipa::path(
    post,
    path = "/refresh/keys",
    tag = "keys",
    responses(
        (status = 200, description = "Keys refreshed successfully", body = RefreshKeysResponse),
        (status = 500, description = "Internal server error")
    )
)]
async fn refresh_keys(
    State(state): State<Arc<KeysState>>,
) -> Result<JsonResponse<RefreshKeysResponse>, (StatusCode, String)> {
    state
        .keys_store
        .load_keys()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(JsonResponse(RefreshKeysResponse {
        status: "ok".to_string(),
    }))
}

// ---------- РОУТЕР ----------
pub async fn init_keys_router(connections: Arc<Connections>) -> Result<Router> {
    let repo_impl = Arc::new(VaultKeysRepo::new(connections.vault_conn.clone()));
    let key_store = Arc::new(KeyStore::new(repo_impl));
    key_store.load_keys().await?;

    let state = Arc::new(KeysState {
        keys_store: key_store,
    });

    let router = Router::new()
        .route("/.well-known/jwks.json", get(jwks))
        .route("/sign/jwt", post(sign_jwt_token))
        .route("/refresh/keys", post(refresh_keys))
        .with_state(state);

    Ok(router)
}
