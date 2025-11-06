use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use p256::ecdsa::{Signature, signature::Signer};
use serde_json::{Value, json};

use crate::keys::key_store::KeyEntry;

pub struct JwtService;

impl JwtService {
    pub fn sign_jwt(
        payload: Value,
        key_entry: &KeyEntry,
    ) -> Result<String, Box<dyn std::error::Error>> {
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
