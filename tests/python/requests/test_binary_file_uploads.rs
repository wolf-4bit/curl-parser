use curl_parser::converters::python::Requests;
use curl_parser::converters::Convert;
use curl_parser::parse_curl_command;

#[test]
fn test_basic_binary_upload() {
    let cmd = "curl --data-binary @/path/to/file.bin https://api.example.com/upload";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.binary_file, Some("/path/to/file.bin".to_string()));

    
    assert_eq!(parsed.method, "POST");

    
    let converter = Requests;
    let python_code = converter.convert(&parsed);

    
    assert!(python_code.contains("with open(\"/path/to/file.bin\", \"rb\") as f:"));
    assert!(python_code.contains("binary_data = f.read()"));
    assert!(python_code.contains("response = requests.post("));
    assert!(python_code.contains("data=binary_data"));
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

    
    let converter = Requests;
    let python_code = converter.convert(&parsed);

    
    assert!(python_code.contains("with open(\"/path/to/document.pdf\", \"rb\") as f:"));
    assert!(python_code.contains("binary_data = f.read()"));
    assert!(python_code.contains("response = requests.put("));
}

#[test]
fn test_binary_upload_with_headers() {
    let cmd = "curl --data-binary @/path/to/image.jpg -H \"Content-Type: image/jpeg\" -H \"X-API-Key: abc123\" https://api.example.com/images";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.binary_file, Some("/path/to/image.jpg".to_string()));
    assert_eq!(parsed.headers.get("Content-Type").unwrap(), "image/jpeg");
    assert_eq!(parsed.headers.get("X-API-Key").unwrap(), "abc123");

    
    let converter = Requests;
    let python_code = converter.convert(&parsed);

    
    assert!(python_code.contains("headers = {"));
    assert!(python_code.contains("\"Content-Type\": \"image/jpeg\""));
    assert!(python_code.contains("\"X-API-Key\": \"abc123\""));
    assert!(python_code.contains("with open(\"/path/to/image.jpg\", \"rb\") as f:"));
}

#[test]
fn test_space_in_filepath() {
    
    let cmd = "curl --data-binary @\"/path/with spaces/file.dat\" https://api.example.com/upload";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(
        parsed.binary_file,
        Some("/path/with spaces/file.dat".to_string())
    );

    
    let converter = Requests;
    let python_code = converter.convert(&parsed);

    
    assert!(python_code.contains("with open(\"/path/with spaces/file.dat\", \"rb\") as f:"));
}

#[test]
fn test_binary_with_auth_and_options() {
    
    let cmd = "curl --data-binary @/path/to/file.bin --oauth2-bearer TOKEN123 --compressed https://api.example.com/upload";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.binary_file, Some("/path/to/file.bin".to_string()));
    assert_eq!(parsed.oauth2_bearer, Some("TOKEN123".to_string()));
    assert!(parsed.compressed);

    
    let converter = Requests;
    let python_code = converter.convert(&parsed);

    
    assert!(python_code.contains("with open(\"/path/to/file.bin\", \"rb\") as f:"));
    assert!(python_code.contains("\"Authorization\": \"Bearer TOKEN123\""));
}

#[test]
fn test_data_binary_without_file() {
    
    let cmd = "curl --data-binary '{\"data\":\"raw\"}' https://api.example.com/endpoint";
    let parsed = parse_curl_command(cmd).unwrap();

    
    assert_eq!(parsed.binary_file, None);
    assert_eq!(parsed.data, Some("{\"data\":\"raw\"}".to_string()));

    
    let converter = Requests;
    let python_code = converter.convert(&parsed);

    
    assert!(!python_code.contains("with open("));
    assert!(!python_code.contains("binary_data = f.read()"));

    
    assert!(python_code.contains("data =") || python_code.contains("data="));
}
