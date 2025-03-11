use curl_parser::converters::nodejs::Axios;
use curl_parser::converters::Convert;
use curl_parser::parse_curl_command;

#[test]
fn test_standard_http_methods() {
    
    let methods = vec!["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];

    for method in methods {
        let cmd = format!("curl -X {} https://api.example.com/resource", method);
        let parsed = parse_curl_command(&cmd).unwrap();

        
        assert_eq!(parsed.method, method);

        
        let converter = Axios;
        let js_code = converter.convert(&parsed);

        
        let method_lower = method.to_lowercase();
        assert!(js_code.contains(&format!("  method: '{}'", method_lower)));
    }
}

#[test]
fn test_nonstandard_method() {
    
    let cmd = "curl -X PROPFIND https://webdav.example.com/resource";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.method, "PROPFIND");

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("  method: 'propfind'"));
}

#[test]
fn test_standard_vs_nonstandard_methods() {
    
    let all_methods = vec![
        
        "GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS",
        
        "PROPFIND", "MKCOL", "COPY", "MOVE", "LOCK", "UNLOCK", "REPORT",
    ];

    for method in &all_methods {
        let cmd = format!("curl -X {} https://api.example.com/resource", method);
        let parsed = parse_curl_command(&cmd).unwrap();
        let converter = Axios;
        let js_code = converter.convert(&parsed);

        
        let method_lower = method.to_lowercase();
        assert!(js_code.contains(&format!("  method: '{}'", method_lower)));
        assert!(js_code.contains("axios(config)"));
    }
}

#[test]
fn test_custom_method_with_data() {
    
    let cmd = "curl -X PATCH -d '{\"field\":\"value\"}' https://api.example.com/resource/123";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.method, "PATCH");
    assert_eq!(parsed.data, Some("{\"field\":\"value\"}".to_string()));

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("  method: 'patch'"));
    
    assert!(js_code.contains("  data: {\"field\":\"value\"},"));
}

#[test]
fn test_custom_method_with_json() {
    
    let cmd = "curl -X PUT --json '{\"updated\":true}' https://api.example.com/items/123";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.method, "PUT");
    assert!(parsed.data_is_json);

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("  method: 'put'"));
    assert!(js_code.contains("  data: {\"updated\":true},"));
}

#[test]
fn test_default_method_behavior() {
    
    let cmd = "curl https://api.example.com/resource";
    let parsed = parse_curl_command(cmd).unwrap();

    assert_eq!(parsed.method, "GET");

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);
    assert!(js_code.contains("  method: 'get'"));

    
    let cmd_with_data = "curl -d 'data' https://api.example.com/resource";
    let parsed_with_data = parse_curl_command(cmd_with_data).unwrap();

    assert_eq!(parsed_with_data.method, "POST");

    
    let js_code_with_data = converter.convert(&parsed_with_data);
    assert!(js_code_with_data.contains("  method: 'post'"));
}
