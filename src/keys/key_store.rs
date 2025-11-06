use anyhow::Result;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use p256::{
    ecdsa::{SigningKey, VerifyingKey},
    pkcs8::DecodePrivateKey,
};
use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::keys::{
    keys_repo::KeysRepoAsync,
    models::{JWK, JwksResponse},
};

#[derive(Debug, Clone)]
pub struct KeyEntry {
    pub signing_key: SigningKey,
    verifying_key: VerifyingKey,
    pub version: u64,
}

type Keys = Mutex<HashMap<String, KeyEntry>>;

/// KeyStore TODO: Create keys rotation
pub struct KeyStore {
    keys: Keys,
    repo: KeysRepoAsync,
}

impl KeyStore {
    pub fn new(repo: KeysRepoAsync) -> Self {
        Self {
            keys: Mutex::new(HashMap::new()),
            repo,
        }
    }

    /// Загрузка ключей
    pub async fn load_keys(&self) -> Result<()> {
        let latest_version = self.repo.get_current_version().await?;
        let mut keys_guard = self.keys.lock().await;

        // Загружаем текущий ключ
        let current_key_pem = self.repo.get_secret(latest_version).await?;
        let current_key = Self::parse_private_key(&current_key_pem)?;

        keys_guard.insert(
            "latest".to_string(),
            KeyEntry {
                signing_key: current_key.0,
                verifying_key: current_key.1,
                version: latest_version,
            },
        );

        if latest_version > 1 {
            let previous_key_pem = self.repo.get_secret(latest_version - 1).await?;
            let previous_key = Self::parse_private_key(&previous_key_pem)?;

            keys_guard.insert(
                "previous".to_string(),
                KeyEntry {
                    signing_key: previous_key.0,
                    verifying_key: previous_key.1,
                    version: latest_version - 1,
                },
            );
        }

        Ok(())
    }

    fn parse_private_key(pem: &str) -> Result<(SigningKey, VerifyingKey)> {
        let signing_key = SigningKey::from_pkcs8_pem(pem)?;
        let verifying_key = VerifyingKey::from(&signing_key);
        Ok((signing_key, verifying_key))
    }

    pub async fn get_actual_private_key(&self) -> Option<KeyEntry> {
        let keys_guard = self.keys.lock().await;
        keys_guard.get("latest").cloned()
    }

    pub async fn get_jwk_list(&self) -> JwksResponse {
        let keys_guard = self.keys.lock().await;
        let mut jwks = Vec::new();

        for (_, key_entry) in keys_guard.iter() {
            jwks.push(key_entry.to_jwk());
        }

        JwksResponse { keys: jwks }
    }
}

impl KeyEntry {
    fn to_jwk(&self) -> JWK {
        let point = self.verifying_key.to_encoded_point(false);
        let x = URL_SAFE_NO_PAD.encode(point.x().unwrap());
        let y = URL_SAFE_NO_PAD.encode(point.y().unwrap());

        JWK {
            kty: "EC".to_string(),
            r#use: "sig".to_string(),
            kid: self.version.to_string(),
            alg: "ES256".to_string(),
            crv: "P-256".to_string(),
            x,
            y,
        }
    }
}
