use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct JwksResponse {
    pub keys: Vec<JWK>,
}

#[derive(Debug, Serialize)]
pub struct JWK {
    pub kty: String,
    pub r#use: String,
    pub kid: String,
    pub alg: String,
    pub crv: String,
    pub x: String,
    pub y: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshKeysResponse {
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct SignJwtTokenResponse {
    pub token: String,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateJwtRequest {
    #[schema(value_type = Object)]
    pub payload: serde_json::Value,
}
