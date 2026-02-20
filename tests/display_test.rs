/// Tests for display/formatting logic that does not touch the network.

#[test]
fn json_pretty_round_trips() {
    let input = r#"{"name":"Alice","age":30,"active":true}"#;
    let v: serde_json::Value = serde_json::from_str(input).expect("valid JSON");
    let pretty = serde_json::to_string_pretty(&v).expect("serialises");
    assert!(pretty.contains("\"name\""));
    assert!(pretty.contains("\"Alice\""));
    assert!(pretty.contains("\"age\""));
}

#[test]
fn status_code_class() {
    fn class(code: u16) -> &'static str {
        match code {
            200..=299 => "success",
            300..=399 => "redirect",
            400..=499 => "client_error",
            500..=599 => "server_error",
            _         => "unknown",
        }
    }
    assert_eq!(class(200), "success");
    assert_eq!(class(204), "success");
    assert_eq!(class(301), "redirect");
    assert_eq!(class(404), "client_error");
    assert_eq!(class(422), "client_error");
    assert_eq!(class(500), "server_error");
    assert_eq!(class(503), "server_error");
}

#[test]
fn content_type_json_detection() {
    let is_json = |ct: &str| {
        ct.contains("application/json") || ct.contains("text/json")
    };
    assert!(is_json("application/json"));
    assert!(is_json("application/json; charset=utf-8"));
    assert!(is_json("text/json"));
    assert!(!is_json("text/html"));
    assert!(!is_json("application/xml"));
}
