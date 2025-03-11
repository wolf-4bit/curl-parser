use curl_parser::converters::nodejs::Axios;
use curl_parser::converters::Convert;
use curl_parser::parse_curl_command;

#[test]
fn test_basic_oauth2_bearer() {
    let cmd = "curl --oauth2-bearer TOKEN123 https://api.example.com/resource";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.oauth2_bearer, Some("TOKEN123".to_string()));

    
    assert_eq!(
        parsed.headers.get("Authorization").unwrap(),
        "Bearer TOKEN123"
    );

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("  headers: {"));
    assert!(js_code.contains("    'Authorization': 'Bearer TOKEN123',"));
}

#[test]
fn test_oauth2_with_complex_token() {
    
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
    let cmd = format!(
        "curl --oauth2-bearer {} https://api.example.com/protected",
        token
    );

    let parsed = parse_curl_command(&cmd).unwrap();

    
    assert_eq!(parsed.oauth2_bearer, Some(token.to_string()));

    
    let expected_header = format!("Bearer {}", token);
    assert_eq!(
        parsed.headers.get("Authorization").unwrap().as_str(),
        expected_header
    );

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains(&format!("    'Authorization': 'Bearer {}',", token)));
}

#[test]
fn test_oauth2_with_other_headers() {
    let cmd = "curl --oauth2-bearer TOKEN123 -H \"X-API-Key: abc123\" -H \"Content-Type: application/json\" https://api.example.com/resource";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.oauth2_bearer, Some("TOKEN123".to_string()));
    assert_eq!(
        parsed.headers.get("Authorization").unwrap(),
        "Bearer TOKEN123"
    );
    assert_eq!(parsed.headers.get("X-API-Key").unwrap(), "abc123");
    assert_eq!(
        parsed.headers.get("Content-Type").unwrap(),
        "application/json"
    );

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("    'Authorization': 'Bearer TOKEN123',"));
    assert!(js_code.contains("    'X-API-Key': 'abc123',"));
    assert!(js_code.contains("    'Content-Type': 'application/json',"));
}

#[test]
fn test_oauth2_with_explicit_auth_header() {
    
    
    let cmd = "curl --oauth2-bearer TOKEN123 -H \"Authorization: Basic YWxhZGRpbjpvcGVuc2VzYW1l\" https://api.example.com/resource";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.oauth2_bearer, Some("TOKEN123".to_string()));

    
    

    
    let converter = Axios;
    let _js_code = converter.convert(&parsed);

    
    
    
}

#[test]
fn test_oauth2_with_json_data() {
    
    let cmd =
        "curl --oauth2-bearer TOKEN123 --json '{\"name\":\"John\"}' https://api.example.com/users";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.oauth2_bearer, Some("TOKEN123".to_string()));
    assert!(parsed.data_is_json);
    assert_eq!(parsed.data, Some("{\"name\":\"John\"}".to_string()));

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("    'Authorization': 'Bearer TOKEN123',"));
    assert!(js_code.contains("  data: {\"name\":\"John\"},"));
}
