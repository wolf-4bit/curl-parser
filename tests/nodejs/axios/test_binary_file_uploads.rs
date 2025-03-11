use curl_parser::converters::nodejs::Axios;
use curl_parser::converters::Convert;
use curl_parser::parse_curl_command;

#[test]
fn test_basic_binary_upload() {
    let cmd = "curl --data-binary @/path/to/file.bin https://api.example.com/upload";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.binary_file, Some("/path/to/file.bin".to_string()));

    
    assert_eq!(parsed.method, "POST");

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("const fs = require('fs');"));
    assert!(js_code.contains("const binaryData = fs.readFileSync('/path/to/file.bin');"));
    assert!(js_code.contains("  method: 'post',"));
    assert!(js_code.contains("  data: binaryData,"));
    assert!(js_code.contains("    'Content-Type': 'application/octet-stream',"));
}

#[test]
fn test_binary_upload_with_custom_method() {
    let cmd = "curl -X PUT --data-binary @/path/to/document.pdf https://api.example.com/files/123";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(
        parsed.binary_file,
        Some("/path/to/document.pdf".to_string())
    );
    assert_eq!(parsed.method, "PUT");

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("const binaryData = fs.readFileSync('/path/to/document.pdf');"));
    assert!(js_code.contains("  method: 'put',"));
    assert!(js_code.contains("  data: binaryData,"));
}

#[test]
fn test_binary_upload_with_headers() {
    let cmd = "curl --data-binary @/path/to/image.jpg -H \"Content-Type: image/jpeg\" -H \"X-API-Key: abc123\" https://api.example.com/images";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.binary_file, Some("/path/to/image.jpg".to_string()));
    assert_eq!(parsed.headers.get("Content-Type").unwrap(), "image/jpeg");
    assert_eq!(parsed.headers.get("X-API-Key").unwrap(), "abc123");

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("const binaryData = fs.readFileSync('/path/to/image.jpg');"));
    assert!(js_code.contains("  headers: {"));
    assert!(js_code.contains("    'Content-Type': 'image/jpeg',"));
    assert!(js_code.contains("    'X-API-Key': 'abc123',"));
    assert!(js_code.contains("  data: binaryData,"));
}

#[test]
fn test_space_in_filepath() {
    
    let cmd = "curl --data-binary @\"/path/with spaces/file.dat\" https://api.example.com/upload";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(
        parsed.binary_file,
        Some("/path/with spaces/file.dat".to_string())
    );

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("const binaryData = fs.readFileSync('/path/with spaces/file.dat');"));
}

#[test]
fn test_binary_with_auth_and_options() {
    
    let cmd = "curl --data-binary @/path/to/file.bin --oauth2-bearer TOKEN123 --compressed https://api.example.com/upload";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.binary_file, Some("/path/to/file.bin".to_string()));
    assert_eq!(parsed.oauth2_bearer, Some("TOKEN123".to_string()));
    assert!(parsed.compressed);

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(js_code.contains("const binaryData = fs.readFileSync('/path/to/file.bin');"));
    assert!(js_code.contains("    'Authorization': 'Bearer TOKEN123',"));
    assert!(js_code.contains("  data: binaryData,"));
}

#[test]
fn test_data_binary_without_file() {
    
    let cmd = "curl --data-binary '{\"data\":\"raw\"}' https://api.example.com/endpoint";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.binary_file, None);
    assert_eq!(parsed.data, Some("{\"data\":\"raw\"}".to_string()));

    
    let converter = Axios;
    let js_code = converter.convert(&parsed);

    
    assert!(!js_code.contains("fs.readFileSync"));
    assert!(!js_code.contains("binaryData"));

    
    assert!(js_code.contains("  data:"));
}
