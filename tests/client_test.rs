use rustrest::request::{HttpMethod, HttpRequest};
use std::collections::HashMap;

#[test]
fn request_stores_method_and_url() {
    let req = HttpRequest {
        method:  HttpMethod::Get,
        url:     "https://httpbin.org/get".to_string(),
        headers: HashMap::new(),
        query:   HashMap::new(),
        body:    None,
    };
    assert_eq!(req.url, "https://httpbin.org/get");
    assert!(req.headers.is_empty());
    assert!(req.body.is_none());
}

#[test]
fn http_method_display() {
    assert_eq!(HttpMethod::Get.to_string(),    "GET");
    assert_eq!(HttpMethod::Post.to_string(),   "POST");
    assert_eq!(HttpMethod::Put.to_string(),    "PUT");
    assert_eq!(HttpMethod::Patch.to_string(),  "PATCH");
    assert_eq!(HttpMethod::Delete.to_string(), "DELETE");
    assert_eq!(HttpMethod::Head.to_string(),   "HEAD");
}

#[test]
fn request_with_headers() {
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer token123".to_string());
    headers.insert("Accept".to_string(), "application/json".to_string());

    let req = HttpRequest {
        method:  HttpMethod::Post,
        url:     "https://api.example.com/users".to_string(),
        headers,
        query:   HashMap::new(),
        body:    None,
    };

    assert_eq!(req.headers.get("Authorization").unwrap(), "Bearer token123");
    assert_eq!(req.headers.len(), 2);
}
