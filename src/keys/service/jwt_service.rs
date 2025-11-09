use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use p256::ecdsa::{Signature, signature::Signer};
use serde_json::{Value, json};

use super::key_store::KeyEntry;
use anyhow::Result;

pub struct JwtService;

impl JwtService {
    pub fn sign_jwt(payload: Value, key_entry: &KeyEntry) -> Result<String> {
        // Создаем заголовок JWT
        let header = json!({
            "typ": "JWT",
            "alg": "ES256",
            "kid": key_entry.version.to_string()
        });

        // Кодируем header и payload в base64
        let header_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_string(&header)?);
        let payload_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_string(&payload)?);

        // Создаем данные для подписи
        let signing_input = format!("{}.{}", header_b64, payload_b64);

        // Подписываем
        let signature: Signature = key_entry.signing_key.sign(signing_input.as_bytes());

        // Конвертируем сигнатуру в bytes
        let signature_b64 = URL_SAFE_NO_PAD.encode(signature.to_bytes());

        // Собираем итоговый токен
        let token = format!("{}.{}.{}", header_b64, payload_b64, signature_b64);
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keys::service::key_store::KeyEntry;
    use p256::ecdsa::SigningKey;
    use p256::pkcs8::DecodePrivateKey;
    use serde_json::json;

    const TEST_PEM: &str = r#"-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgJN9RhxGesEv9Mo0W
xm77Eh8RMIz/WsfzWT7njD3us/2hRANCAATQ7H8XoOgOmfqofR1SzTME+3lLIlw+
DJ/ShGcPVJlrBTWrsGUIkNXgQlrZG0jAF/1y1KDwhrwuij66EEIMeELN
-----END PRIVATE KEY-----"#;

    #[tokio::test]
    async fn test_sign_jwt() {
        let signing_key = SigningKey::from_pkcs8_pem(TEST_PEM).unwrap();
        let verifying_key = signing_key.verifying_key().clone();
        let key_entry = KeyEntry {
            signing_key,
            verifying_key,
            version: 1,
        };

        let payload = json!({"sub": "1234567890", "name": "John Doe"});
        let token = JwtService::sign_jwt(payload, &key_entry).unwrap();
        println!("{token}");

        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);
        assert!(!parts[0].is_empty());
        assert!(!parts[1].is_empty());
        assert!(!parts[2].is_empty());

        assert!(parts[0].starts_with("ey"));
        assert!(token.eq("eyJhbGciOiJFUzI1NiIsImtpZCI6IjEiLCJ0eXAiOiJKV1QifQ.eyJuYW1lIjoiSm9obiBEb2UiLCJzdWIiOiIxMjM0NTY3ODkwIn0.Yy6vcEzVFApsnhP1D8fFGYPRO0ayTkhp4F4gZQgezrRbG9jgFUYI5sJ1xLvdF76qvpvqcRKqFhcPD2Ns_6I3Pw"))
    }
}
