mod common;

use common::{create_crate, create_rustacean, delete_crate, delete_rustacean};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::{json, Value};

#[test]
fn test_create_crate() {
    let client = Client::new();
    let rustacean_id = create_rustacean(&client);

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
    assert_eq!(result["rustacean_id"].as_i64().unwrap(), rustacean_id);
    assert_eq!(result["code"].as_str().unwrap(), "test-crate");
    assert_eq!(result["name"].as_str().unwrap(), "Test Crate");
    assert_eq!(result["version"].as_str().unwrap(), "0.1.0");
    assert_eq!(result["description"].as_str().unwrap(), "Test Description");
    assert!(result["created_at"].as_str().is_some());

    let crate_id = result["id"].as_i64().unwrap();
    delete_crate(&client, crate_id);
    delete_rustacean(&client, rustacean_id);
}

#[test]
fn test_get_crates() {
    let client = Client::new();
    let rustacean_id = create_rustacean(&client);
    let crate_id = create_crate(&client, rustacean_id);

    let response = client.get("http://localhost:8000/crates").send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let list = response.json::<Value>().unwrap();
    let crates = list.as_array().expect("crates response is an array");
    let our_crate = crates
        .iter()
        .find(|c| c["id"].as_i64() == Some(crate_id))
        .expect("created crate appears in list");

    assert_eq!(our_crate["rustacean_id"].as_i64().unwrap(), rustacean_id);
    assert_eq!(our_crate["code"].as_str().unwrap(), "test-crate");
    assert_eq!(our_crate["name"].as_str().unwrap(), "Test Crate");
    assert_eq!(our_crate["version"].as_str().unwrap(), "0.1.0");
    assert!(our_crate["created_at"].as_str().is_some());

    delete_crate(&client, crate_id);
    delete_rustacean(&client, rustacean_id);
}

#[test]
fn test_get_crate() {
    let client = Client::new();
    let rustacean_id = create_rustacean(&client);
    let crate_id = create_crate(&client, rustacean_id);

    let response = client
        .get(format!("http://localhost:8000/crates/{}", crate_id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let result = response.json::<Value>().unwrap();
    assert_eq!(result["id"].as_i64().unwrap(), crate_id);
    assert_eq!(result["rustacean_id"].as_i64().unwrap(), rustacean_id);
    assert_eq!(result["code"].as_str().unwrap(), "test-crate");
    assert_eq!(result["name"].as_str().unwrap(), "Test Crate");
    assert_eq!(result["version"].as_str().unwrap(), "0.1.0");
    assert_eq!(result["description"].as_str().unwrap(), "Test Description");
    assert!(result["created_at"].as_str().is_some());

    delete_crate(&client, crate_id);
    delete_rustacean(&client, rustacean_id);
}

#[test]
fn test_update_crate() {
    let client = Client::new();
    let rustacean_id = create_rustacean(&client);
    let crate_id = create_crate(&client, rustacean_id);

    let response = client
        .put(format!("http://localhost:8000/crates/{}", crate_id))
        .json(&json!({
            "name": "Updated Crate",
            "version": "0.2.0",
            "description": "Updated Description",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let result = response.json::<Value>().unwrap();
    assert_eq!(result["id"].as_i64().unwrap(), crate_id);
    assert_eq!(result["rustacean_id"].as_i64().unwrap(), rustacean_id);
    assert_eq!(result["code"].as_str().unwrap(), "test-crate");
    assert_eq!(result["name"].as_str().unwrap(), "Updated Crate");
    assert_eq!(result["version"].as_str().unwrap(), "0.2.0");
    assert_eq!(result["description"].as_str().unwrap(), "Updated Description");
    assert!(result["created_at"].as_str().is_some());

    delete_crate(&client, crate_id);
    delete_rustacean(&client, rustacean_id);
}

#[test]
fn test_update_crate_not_found() {
    let client = Client::new();
    let response = client
        .put("http://localhost:8000/crates/9999")
        .json(&json!({}))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_delete_crate() {
    let client = Client::new();
    let rustacean_id = create_rustacean(&client);
    let crate_id = create_crate(&client, rustacean_id);

    let response = client
        .delete(format!("http://localhost:8000/crates/{}", crate_id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = client
        .get(format!("http://localhost:8000/crates/{}", crate_id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    delete_rustacean(&client, rustacean_id);
}
