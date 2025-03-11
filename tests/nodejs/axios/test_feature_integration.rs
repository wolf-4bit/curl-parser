use curl_parser::converters::nodejs::Axios;
use curl_parser::converters::Convert;
use curl_parser::parse_curl_command;

#[test]
fn test_all_features_together() {
    
    let cmd = "curl -X PATCH \
               --oauth2-bearer TOKEN123 \
               --json '{\"updates\":[{\"path\":\"/name\",\"value\":\"Updated\"}]}' \
               -H \"X-API-Key: abc123\" \
               --compressed \
               https://api.example.com/resources/123";

    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.method, "PATCH");
    assert_eq!(parsed.oauth2_bearer, Some("TOKEN123".to_string()));
    assert!(parsed.data_is_json);
    assert_eq!(
        parsed.data,
        Some("{\"updates\":[{\"path\":\"/name\",\"value\":\"Updated\"}]}".to_string())
    );
    assert_eq!(parsed.headers.get("X-API-Key").unwrap(), "abc123");
    assert_eq!(
        parsed.headers.get("Content-Type").unwrap(),
        "application/json"
    );
    assert_eq!(
        parsed.headers.get("Authorization").unwrap(),
        "Bearer TOKEN123"
    );
    assert!(parsed.compressed);

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("  method: 'patch',"));
    assert!(js_code.contains("    'Authorization': 'Bearer TOKEN123',"));
    assert!(js_code.contains("    'X-API-Key': 'abc123',"));
    assert!(js_code.contains("    'Content-Type': 'application/json',"));
    assert!(js_code.contains("  data: {\"updates\":[{\"path\":\"/name\",\"value\":\"Updated\"}]}"));
}

#[test]
fn test_binary_upload_with_oauth2() {
    
    let cmd = "curl -X PUT \
               --data-binary @/path/to/document.pdf \
               --oauth2-bearer TOKEN123 \
               -H \"Content-Type: application/pdf\" \
               https://api.example.com/files/upload";

    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.method, "PUT");
    assert_eq!(
        parsed.binary_file,
        Some("/path/to/document.pdf".to_string())
    );
    assert_eq!(parsed.oauth2_bearer, Some("TOKEN123".to_string()));
    assert_eq!(
        parsed.headers.get("Content-Type").unwrap(),
        "application/pdf"
    );

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("const fs = require('fs');"));
    assert!(js_code.contains("const binaryData = fs.readFileSync('/path/to/document.pdf');"));
    assert!(js_code.contains("  method: 'put',"));
    assert!(js_code.contains("  data: binaryData,"));
    assert!(js_code.contains("    'Authorization': 'Bearer TOKEN123',"));
    assert!(js_code.contains("    'Content-Type': 'application/pdf',"));
}

#[test]
fn test_json_with_custom_method_and_headers() {
    
    let cmd = "curl -X PATCH \
               --json '{\"status\":\"active\"}' \
               -H \"If-Match: \\\"etag123\\\"\" \
               -H \"X-Request-ID: req-123\" \
               https://api.example.com/users/456/status";

    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.method, "PATCH");
    assert!(parsed.data_is_json);
    assert_eq!(parsed.data, Some("{\"status\":\"active\"}".to_string()));
    assert_eq!(parsed.headers.get("If-Match").unwrap(), "\"etag123\"");
    assert_eq!(parsed.headers.get("X-Request-ID").unwrap(), "req-123");
    assert_eq!(
        parsed.headers.get("Content-Type").unwrap(),
        "application/json"
    );

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("  method: 'patch',"));
    assert!(js_code.contains("  data: {\"status\":\"active\"},"));
    
    assert!(js_code.contains("    'X-Request-ID': 'req-123',"));
    assert!(js_code.contains("    'Content-Type': 'application/json',"));
}

#[test]
fn test_all_options_with_complex_url() {
    
    let cmd = "curl -X DELETE \
               --oauth2-bearer TOKEN123 \
               -H \"Accept: application/json\" \
               \"https://api.example.com/resources/nested/path?param1=value1&param2=value2\" \
               --compressed";

    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.method, "DELETE");
    assert_eq!(parsed.oauth2_bearer, Some("TOKEN123".to_string()));
    assert_eq!(parsed.headers.get("Accept").unwrap(), "application/json");
    assert_eq!(
        parsed.url,
        "https://api.example.com/resources/nested/path?param1=value1&param2=value2"
    );
    assert!(parsed.compressed);

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("  method: 'delete',"));
    assert!(js_code.contains(
       "  url: 'https://api.example.com/resources/nested/path?param1=value1&param2=value2',"
    ));
    assert!(js_code.contains("    'Authorization': 'Bearer TOKEN123',"));
    assert!(js_code.contains("    'Accept': 'application/json',"));
}
