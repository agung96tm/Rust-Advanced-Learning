use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::{json, Value};

/// Creates a rustacean via the API and returns its id.
#[allow(dead_code)]
pub fn create_rustacean(client: &Client) -> i64 {
    let response = client
        .post("http://localhost:8000/rustaceans")
        .json(&json!({
            "name": "John Doe",
            "email": "john.doe@example.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let result = response.json::<Value>().unwrap();
    result["id"].as_i64().unwrap()
}

/// Creates a crate via the API and returns its id. Requires an existing rustacean_id.
#[allow(dead_code)]
pub fn create_crate(client: &Client, rustacean_id: i64) -> i64 {
    let response = client
        .post("http://localhost:8000/crates")
        .json(&json!({
            "rustacean_id": rustacean_id,
            "code": "test-crate",
            "name": "Test Crate",
            "version": "0.1.0",
            "description": "Test Description",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let result = response.json::<Value>().unwrap();
    result["id"].as_i64().unwrap()
}

/// Deletes a rustacean by id via the API.
pub fn delete_rustacean(client: &Client, id: i64) {
    let response = client
        .delete(format!("http://localhost:8000/rustaceans/{}", id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

/// Deletes a crate by id via the API. Call this before deleting the rustacean (FK).
#[allow(dead_code)]
pub fn delete_crate(client: &Client, id: i64) {
    let response = client
        .delete(format!("http://localhost:8000/crates/{}", id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
