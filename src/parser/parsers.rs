use crate::parser::error::ParseError;
use crate::{CurlCommand, FileUpload};
use std::collections::HashMap;


pub fn parse_header(headers: &mut HashMap<String, String>, header: &str) -> Result<(), ParseError> {
    let parts: Vec<&str> = header.splitn(2, ':').collect();
    if parts.len() == 2 {
        let key = parts[0].trim();
        let value = parts[1].trim();

        
        if key.to_lowercase() == "user-agent" {
            return Ok(());
        }

        headers.insert(key.to_string(), value.to_string());
        Ok(())
    } else {
        Err(ParseError::ParseFailure(format!(
            "Invalid header format: {}",
            header
        )))
    }
}


pub fn parse_auth(curl_data: &mut CurlCommand, auth_str: &str) -> Result<(), ParseError> {
    let parts: Vec<&str> = auth_str.splitn(2, ':').collect();
    if parts.len() == 2 {
        curl_data.auth = Some((parts[0].to_string(), parts[1].to_string()));
        Ok(())
    } else {
        Err(ParseError::ParseFailure(format!(
            "Invalid auth format: {}",
            auth_str
        )))
    }
}


pub fn parse_cookie(
    cookies: &mut HashMap<String, String>,
    cookie_str: &str,
) -> Result<(), ParseError> {
    
    for cookie_part in cookie_str.split(';') {
        let cookie_parts: Vec<&str> = cookie_part.splitn(2, '=').collect();
        if cookie_parts.len() == 2 {
            let key = cookie_parts[0].trim();
            let value = cookie_parts[1].trim();
            cookies.insert(key.to_string(), value.to_string());
        } else {
            return Err(ParseError::ParseFailure(format!(
                "Invalid cookie format: {}",
                cookie_str
            )));
        }
    }

    Ok(())
}


pub fn parse_form_field(
    form: &mut HashMap<String, String>,
    files: &mut HashMap<String, FileUpload>,
    field_str: &str,
) -> Result<(), ParseError> {
    let parts: Vec<&str> = field_str.splitn(2, '=').collect();
    if parts.len() == 2 {
        let key = parts[0].trim();
        let value = parts[1].trim();

        
        if value.starts_with('@') {
            let file_parts: Vec<&str> = value[1..].split(';').collect();
            let file_path = file_parts[0].trim();

            let mut file_upload = FileUpload {
                path: file_path.to_string(),
                content_type: None,
                filename: None,
            };

            
            for part in file_parts.iter().skip(1) {
                if part.starts_with("type=") {
                    file_upload.content_type = Some(part[5..].to_string());
                } else if part.starts_with("filename=") {
                    file_upload.filename = Some(part[9..].to_string());
                }
            }

            files.insert(key.to_string(), file_upload);
        } else {
            form.insert(key.to_string(), value.to_string());
        }

        Ok(())
    } else {
        Err(ParseError::ParseFailure(format!(
            "Invalid form field format: {}",
            field_str
        )))
    }
}


pub fn parse_url_encoded_param(
    params: &mut HashMap<String, String>,
    param_str: &str,
) -> Result<(), ParseError> {
    let parts: Vec<&str> = param_str.splitn(2, '=').collect();
    if parts.len() == 2 {
        let key = parts[0].trim();
        let value = parts[1].trim();
        params.insert(key.to_string(), value.to_string());
        Ok(())
    } else {
        Err(ParseError::ParseFailure(format!(
            "Invalid URL-encoded parameter format: {}",
            param_str
        )))
    }
}


pub fn parse_proxy_auth(curl_data: &mut CurlCommand, auth_str: &str) -> Result<(), ParseError> {
    let parts: Vec<&str> = auth_str.splitn(2, ':').collect();
    if parts.len() == 2 {
        curl_data.proxy_auth = Some((parts[0].to_string(), parts[1].to_string()));
        Ok(())
    } else {
        Err(ParseError::ParseFailure(format!(
            "Invalid proxy auth format: {}",
            auth_str
        )))
    }
}
