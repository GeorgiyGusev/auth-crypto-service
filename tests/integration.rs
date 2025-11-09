use reqwest::Client;
use serde_json::json;

const BASE_URL: &str = "http://127.0.0.1:3000/auth-crypto-service/api/v1";

#[tokio::test]
#[ignore = "integration test need enviroment"]
async fn test_jwks_endpoint() {
    let client = Client::new();

    let resp = client
        .get(&format!("{}/.well-known/jwks.json", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert!(resp.status().is_success());
    let body: serde_json::Value = resp.json().await.expect("Failed to parse JSON");
    println!("JWKS response: {:#?}", body);
    assert!(body.get("keys").is_some());
}

#[tokio::test]
#[ignore = "integration test need enviroment"]
async fn test_refresh_keys_endpoint() {
    let client = Client::new();

    let resp = client
        .post(&format!("{}/refresh/keys", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert!(resp.status().is_success());
    let body: serde_json::Value = resp.json().await.expect("Failed to parse JSON");
    println!("Refresh keys response: {:#?}", body);
    assert_eq!(body.get("status").unwrap(), "ok");
}

#[tokio::test]
#[ignore = "integration test need enviroment"]
async fn test_sign_jwt_endpoint() {
    let client = Client::new();

    let payload = json!({
        "payload": {
            "sub": "1234567890",
            "name": "John Doe"
        }
    });

    let resp = client
        .post(&format!("{}/sign/jwt", BASE_URL))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    assert!(resp.status().is_success());
    let body: serde_json::Value = resp.json().await.expect("Failed to parse JSON");
    println!("Sign JWT response: {:#?}", body);
    assert!(body.get("token").is_some());
}
