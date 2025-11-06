use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct JwksResponse {
    #[schema(value_type = [JWK])]
    pub keys: Vec<JWK>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
#[schema(
    title = "JSON Web Key",
    description = "A JSON Web Key (JWK) that represents a cryptographic key"
)]
pub struct JWK {
    pub kty: String,
    pub r#use: String,
    pub kid: String,
    pub alg: String,
    pub crv: String,
    pub x: String,
    pub y: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
#[schema(
    title = "Refresh Keys Response",
    description = "Success response after refreshing keys"
)]
pub struct RefreshKeysResponse {
    pub status: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
#[schema(
    title = "JWT Token Response",
    description = "Response containing a generated JWT token"
)]
pub struct SignJwtTokenResponse {
    /// JWT token
    ///
    /// A signed JSON Web Token. The token's payload and purpose depend on the context
    /// in which it was generated. It follows the standard JWT format with header,
    /// payload, and signature sections.
    ///
    /// **Format:** Three base64-encoded segments separated by dots
    /// **Structure:** `header.payload.signature`
    #[schema(
        example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
    )]
    pub token: String,
}
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateJwtRequest {
    #[schema(value_type = Object)]
    pub payload: serde_json::Value,
}
