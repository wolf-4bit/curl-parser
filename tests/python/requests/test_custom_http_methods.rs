use curl_parser::converters::python::Requests;
use curl_parser::converters::Convert;
use curl_parser::parse_curl_command;

#[test]
fn test_standard_http_methods() {
    
    let methods = vec!["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];

    for method in methods {
        let cmd = format!("curl -X {} https://api.example.com/resource", method);
        let parsed = parse_curl_command(&cmd).unwrap();

        
        assert_eq!(parsed.method, method);

        
        let converter = Requests;
        let python_code = converter.convert(&parsed);

        
        let method_lower = method.to_lowercase();
        assert!(python_code.contains(&format!("response = requests.{}(", method_lower)));
    }
}

#[test]
fn test_custom_method_with_data() {
    
    let cmd = "curl -X PATCH -d '{\"field\":\"value\"}' https://api.example.com/resource/123";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.method, "PATCH");
    assert_eq!(parsed.data, Some("{\"field\":\"value\"}".to_string()));

    
    let converter = Requests;
    let python_code = converter.convert(&parsed);

    
    assert!(python_code.contains("response = requests.patch("));
}

#[test]
fn test_custom_method_with_json() {
    
    let cmd = "curl -X PUT --json '{\"updated\":true}' https://api.example.com/items/123";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.method, "PUT");
    assert!(parsed.data_is_json);

    
    let converter = Requests;
    let python_code = converter.convert(&parsed);

    
    assert!(python_code.contains("response = requests.put("));
    assert!(python_code.contains("json=json_data"));
}

#[test]
fn test_custom_method_with_form() {
    
    let cmd = "curl -X DELETE -F \"confirm=true\" https://api.example.com/users/123";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.method, "DELETE");
    assert_eq!(parsed.form.get("confirm").unwrap(), "true");

    
    let converter = Requests;
    let python_code = converter.convert(&parsed);

    
    assert!(python_code.contains("response = requests.delete("));
    assert!(python_code.contains("files=files"));
}

#[test]
fn test_custom_method_with_binary_file() {
    
    let cmd = "curl -X PUT --data-binary @/path/to/file.dat https://api.example.com/files/upload";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.method, "PUT");
    assert_eq!(parsed.binary_file, Some("/path/to/file.dat".to_string()));

    
    let converter = Requests;
    let python_code = converter.convert(&parsed);

    
    assert!(python_code.contains("response = requests.put("));
    assert!(python_code.contains("with open(\"/path/to/file.dat\", \"rb\") as f:"));
    assert!(python_code.contains("data=binary_data"));
}

#[test]
fn test_default_method_behavior() {
    
    let cmd = "curl https://api.example.com/resource";
    let parsed = parse_curl_command(cmd).unwrap();

    assert_eq!(parsed.method, "GET");

    
    let cmd_with_data = "curl -d 'data' https://api.example.com/resource";
    let parsed_with_data = parse_curl_command(cmd_with_data).unwrap();

    assert_eq!(parsed_with_data.method, "POST");
}

#[test]
fn test_nonstandard_method() {
    
    let cmd = "curl -X PROPFIND https://webdav.example.com/resource";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.method, "PROPFIND");

    
    let converter = Requests;
    let python_code = converter.convert(&parsed);

    
    assert!(python_code.contains(
        "response = requests.request(\"propfind\", \"https://webdav.example.com/resource\""
    ));

    
    assert!(!python_code.contains("response = requests.propfind("));
}

#[test]
fn test_standard_vs_nonstandard_methods() {
    
    let standard_methods = vec!["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];

    for method in &standard_methods {
        let cmd = format!("curl -X {} https://api.example.com/resource", method);
        let parsed = parse_curl_command(&cmd).unwrap();
        let converter = Requests;
        let python_code = converter.convert(&parsed);

        
        let method_lower = method.to_lowercase();
        assert!(python_code.contains(&format!("response = requests.{}(", method_lower)));
    }

    
    let nonstandard_methods = vec![
        "PROPFIND", "MKCOL", "COPY", "MOVE", "LOCK", "UNLOCK", "REPORT",
    ];

    for method in &nonstandard_methods {
        let cmd = format!("curl -X {} https://api.example.com/resource", method);
        let parsed = parse_curl_command(&cmd).unwrap();
        let converter = Requests;
        let python_code = converter.convert(&parsed);

        
        let method_lower = method.to_lowercase();
        assert!(python_code.contains(&format!(
            "response = requests.request(\"{}\", ",
            method_lower
        )));

        
        assert!(!python_code.contains(&format!("response = requests.{}(", method_lower)));
    }
}
