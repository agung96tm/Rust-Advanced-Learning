mod common;

use common::{create_rustacean, delete_rustacean};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::{json, Value};

#[test]
fn test_get_rustaceans() {
    let client = Client::new();

    let rustacean1 = create_rustacean(&client);
    let rustacean2 = create_rustacean(&client);
    let rustacean3 = create_rustacean(&client);

    let response = client.get("http://localhost:8000/rustaceans").send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let result = response.json::<Value>().unwrap();
    let rustaceans = result.as_array().expect("Expected an array");
    assert!(rustaceans.iter().any(|r| {
        r["id"].as_i64().unwrap() == rustacean1
    }));
    assert!(rustaceans.iter().any(|r| {
        r["id"].as_i64().unwrap() == rustacean2
    }));
    assert!(rustaceans.iter().any(|r| {
        r["id"].as_i64().unwrap() == rustacean3
    }));

    delete_rustacean(&client, rustacean1);
    delete_rustacean(&client, rustacean2);
    delete_rustacean(&client, rustacean3);
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let response = client.post("http://localhost:8000/rustaceans")
        .json(&json!({
            "name": "John Doe",
            "email": "john.doe@example.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let result = response.json::<Value>().unwrap();
    assert_eq!(result["name"].as_str().unwrap(), "John Doe");
    assert_eq!(result["email"].as_str().unwrap(), "john.doe@example.com");

    delete_rustacean(&client, result["id"].as_i64().unwrap());
}

#[test]
fn test_view_rustacean() {
    let client = Client::new();
    let id = create_rustacean(&client);
    let response = client.get(&format!("http://localhost:8000/rustaceans/{}", id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    delete_rustacean(&client, id);
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let id = create_rustacean(&client);
    let response = client.put(&format!("http://localhost:8000/rustaceans/{}", id))
        .json(&json!({
            "name": "Jane Fix",
            "email": "jane.fix@example.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let result = response.json::<Value>().unwrap();
    assert_eq!(result["name"].as_str().unwrap(), "Jane Fix");
    assert_eq!(result["email"].as_str().unwrap(), "jane.fix@example.com");

    delete_rustacean(&client, id);
}

#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let id = create_rustacean(&client);
    delete_rustacean(&client, id);

    let response = client.get(&format!("http://localhost:8000/rustaceans/{}", id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
