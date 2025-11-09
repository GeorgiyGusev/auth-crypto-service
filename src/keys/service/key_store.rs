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
    models::{JWK, JwksResponse},
    repo::keys_repo::KeysRepo,
};

#[derive(Debug, Clone)]
pub struct KeyEntry {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    pub version: u64,
}

type Keys = Mutex<HashMap<String, KeyEntry>>;

pub struct KeyStore<R: KeysRepo + Send + Sync> {
    keys: Keys,
    repo: R,
}

impl<R: KeysRepo + Send + Sync> KeyStore<R> {
    pub fn new(repo: R) -> Self {
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

// TODO: remove unwrap, make correct error handling and option handling
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

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use mockall::{mock, predicate::*};

    // ---- 1. Мокаем KeysRepo ----
    mock! {
        pub KeysRepo {}
        #[async_trait]
        impl KeysRepo for KeysRepo {
            async fn get_current_version(&self) -> Result<u64>;
            async fn get_secret(&self, version: u64) -> Result<String>;
        }
    }

    const TEST_PEM: &str = r#"-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgJN9RhxGesEv9Mo0W
xm77Eh8RMIz/WsfzWT7njD3us/2hRANCAATQ7H8XoOgOmfqofR1SzTME+3lLIlw+
DJ/ShGcPVJlrBTWrsGUIkNXgQlrZG0jAF/1y1KDwhrwuij66EEIMeELN
-----END PRIVATE KEY-----
"#;

    #[tokio::test]
    async fn test_load_keys_single_version() {
        let mut mock_repo = MockKeysRepo::new();
        mock_repo.expect_get_current_version().returning(|| Ok(1));
        mock_repo
            .expect_get_secret()
            .with(eq(1))
            .returning(|_| Ok(TEST_PEM.to_string()));

        let store = KeyStore::new(mock_repo);

        // Загрузка ключей
        assert!(store.load_keys().await.is_ok());

        // Проверяем актуальный ключ
        let latest = store.get_actual_private_key().await;
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().version, 1);
    }

    #[tokio::test]
    async fn test_load_keys_two_versions() {
        let mut mock_repo = MockKeysRepo::new();
        mock_repo.expect_get_current_version().returning(|| Ok(2));
        mock_repo
            .expect_get_secret()
            .with(eq(2))
            .returning(|_| Ok(TEST_PEM.to_string()));
        mock_repo
            .expect_get_secret()
            .with(eq(1))
            .returning(|_| Ok(TEST_PEM.to_string()));

        let store = KeyStore::new(mock_repo);

        assert!(store.load_keys().await.is_ok());

        let keys_guard = store.keys.lock().await;
        assert!(keys_guard.contains_key("latest"));
        assert!(keys_guard.contains_key("previous"));
    }

    #[tokio::test]
    async fn test_get_jwk_list_returns_correct_format() {
        let mut mock_repo = MockKeysRepo::new();
        mock_repo.expect_get_current_version().returning(|| Ok(1));
        mock_repo
            .expect_get_secret()
            .returning(|_| Ok(TEST_PEM.to_string()));

        let store = KeyStore::new(mock_repo);
        store.load_keys().await.unwrap();

        let jwks = store.get_jwk_list().await;
        assert_eq!(jwks.keys.len(), 1);

        let jwk = &jwks.keys[0];
        assert_eq!(jwk.kty, "EC");
        assert_eq!(jwk.alg, "ES256");
        assert_eq!(jwk.crv, "P-256");
        assert_eq!(jwk.r#use, "sig");
    }
}
