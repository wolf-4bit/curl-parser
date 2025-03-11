use curl_parser::converters::nodejs::Axios;
use curl_parser::converters::Convert;
use curl_parser::parse_curl_command;

#[test]
fn test_basic_json_option() {
    let cmd = "curl --json '{\"name\":\"John\",\"age\":30}' https://api.example.com/users";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.method, "POST");
    assert_eq!(parsed.url, "https://api.example.com/users");
    assert_eq!(
        parsed.data,
        Some("{\"name\":\"John\",\"age\":30}".to_string())
    );
    assert!(parsed.data_is_json);
    assert_eq!(
        parsed.headers.get("Content-Type").unwrap(),
        "application/json"
    );

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("const axios = require('axios');"));
    assert!(js_code.contains("const config = {"));
    assert!(js_code.contains("  method: 'post',"));
    assert!(js_code.contains("  url: 'https://api.example.com/users',"));
    assert!(js_code.contains("  headers: {"));
    assert!(js_code.contains("    'Content-Type': 'application/json',"));
    assert!(js_code.contains("  data: {\"name\":\"John\",\"age\":30},"));
    assert!(js_code.contains("axios(config)"));
}

#[test]
fn test_json_with_explicit_method() {
    let cmd = "curl -X PATCH --json '{\"name\":\"Updated\"}' https://api.example.com/users/123";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.method, "PATCH");
    assert_eq!(parsed.url, "https://api.example.com/users/123");
    assert_eq!(parsed.data, Some("{\"name\":\"Updated\"}".to_string()));
    assert!(parsed.data_is_json);

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("  method: 'patch',"));
    assert!(js_code.contains("  data: {\"name\":\"Updated\"},"));
}

#[test]
fn test_complex_json_structure() {
    let cmd = "curl --json '{\"user\":{\"name\":\"John\",\"address\":{\"city\":\"New York\"}},\"items\":[1,2,3]}' https://api.example.com/data";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert!(parsed.data_is_json);
    assert_eq!(
        parsed.data,
        Some(
            "{\"user\":{\"name\":\"John\",\"address\":{\"city\":\"New York\"}},\"items\":[1,2,3]}"
                .to_string()
        )
    );

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("  data: {\"user\":{\"name\":\"John\",\"address\":{\"city\":\"New York\"}},\"items\":[1,2,3]},"));
}

#[test]
fn test_json_with_other_options() {
    let cmd = "curl --json '{\"data\":\"test\"}' -H \"X-API-Key: 12345\" --compressed https://api.example.com/data";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert!(parsed.data_is_json);
    assert_eq!(parsed.headers.get("X-API-Key").unwrap(), "12345");
    assert!(parsed.compressed);

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("  data: {\"data\":\"test\"},"));
    assert!(js_code.contains("    'X-API-Key': '12345',"));
}

#[test]
fn test_malformed_json_still_handled() {
    
    
    let cmd = "curl --json '{name:John}' https://api.example.com/users"; // Missing quotes around keys
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert!(parsed.data_is_json);
    assert_eq!(parsed.data, Some("{name:John}".to_string()));

    
    let converter = Axios;
    let _ = converter.convert(&parsed); // Should not panic
}
