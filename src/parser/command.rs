use crate::parser::error::ParseError;
use crate::parser::parsers::{
    parse_auth, parse_cookie, parse_form_field, parse_header, parse_proxy_auth,
    parse_url_encoded_param,
};
use crate::parser::tokenizer::tokenize_command;
use crate::{AuthType, CurlCommand};
use url::Url;


pub fn parse_curl_command(command: &str) -> Result<CurlCommand, ParseError> {
    let mut curl_data = CurlCommand::default();

    
    let tokens = tokenize_command(command)?;

    
    let skip_count = if tokens.get(0).map_or(false, |t| t == "curl") {
        1
    } else {
        0
    };
    let tokens_iter = tokens.iter().skip(skip_count);

    
    let mut i = 0;
    let tokens_vec: Vec<&String> = tokens_iter.collect();

    while i < tokens_vec.len() {
        let token = tokens_vec[i];

        match token.as_str() {
            "-X" | "--request" => {
                if i + 1 < tokens_vec.len() {
                    
                    let method = tokens_vec[i + 1].to_uppercase();
                    match method.as_str() {
                        "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" | "OPTIONS"
                        | "TRACE" | "CONNECT" => {
                            curl_data.method = method;
                        }
                        _ => {
                            
                            curl_data.method = method;
                        }
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-H" | "--header" => {
                if i + 1 < tokens_vec.len() {
                    parse_header(&mut curl_data.headers, tokens_vec[i + 1])?;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-d" | "--data" | "--data-ascii" | "--data-binary" => {
                if i + 1 < tokens_vec.len() {
                    
                    if token == "--data-binary" && tokens_vec[i + 1].starts_with('@') {
                        curl_data.binary_file = Some(tokens_vec[i + 1][1..].to_string());
                    } else {
                        curl_data.data = Some(tokens_vec[i + 1].clone());
                    }

                    
                    
                    if curl_data.method == "GET" {
                        curl_data.method = "POST".to_string();
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--data-raw" => {
                if i + 1 < tokens_vec.len() {
                    curl_data.data = Some(tokens_vec[i + 1].clone());
                    
                    if curl_data.method == "GET" {
                        curl_data.method = "POST".to_string();
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--json" => {
                if i + 1 < tokens_vec.len() {
                    curl_data.data = Some(tokens_vec[i + 1].clone());
                    curl_data.data_is_json = true;
                    curl_data
                        .headers
                        .insert("Content-Type".to_string(), "application/json".to_string());
                    
                    if curl_data.method == "GET" {
                        curl_data.method = "POST".to_string();
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--data-urlencode" => {
                if i + 1 < tokens_vec.len() {
                    parse_url_encoded_param(&mut curl_data.url_encoded_params, tokens_vec[i + 1])?;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-G" | "--get" => {
                
                curl_data.method = "GET".to_string();
                i += 1;
            }
            "-F" | "--form" => {
                if i + 1 < tokens_vec.len() {
                    parse_form_field(&mut curl_data.form, &mut curl_data.files, tokens_vec[i + 1])?;
                    
                    if curl_data.method == "GET" {
                        curl_data.method = "POST".to_string();
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-u" | "--user" => {
                if i + 1 < tokens_vec.len() {
                    parse_auth(&mut curl_data, tokens_vec[i + 1])?;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--digest" => {
                curl_data.auth_type = Some(AuthType::Digest);
                i += 1;
            }
            "--ntlm" => {
                curl_data.auth_type = Some(AuthType::Ntlm);
                i += 1;
            }
            "--negotiate" => {
                curl_data.auth_type = Some(AuthType::Negotiate);
                i += 1;
            }
            "-b" | "--cookie" => {
                if i + 1 < tokens_vec.len() {
                    parse_cookie(&mut curl_data.cookies, tokens_vec[i + 1])?;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-A" | "--user-agent" => {
                if i + 1 < tokens_vec.len() {
                    curl_data.user_agent = Some(tokens_vec[i + 1].clone());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-k" | "--insecure" => {
                curl_data.insecure = true;
                i += 1;
            }
            "--compressed" => {
                curl_data.compressed = true;
                i += 1;
            }
            "-x" | "--proxy" => {
                if i + 1 < tokens_vec.len() {
                    curl_data.proxy = Some(tokens_vec[i + 1].to_string());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--proxy-user" => {
                if i + 1 < tokens_vec.len() {
                    parse_proxy_auth(&mut curl_data, tokens_vec[i + 1])?;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--oauth2-bearer" => {
                if i + 1 < tokens_vec.len() {
                    curl_data.oauth2_bearer = Some(tokens_vec[i + 1].clone());
                    curl_data.headers.insert(
                        "Authorization".to_string(),
                        format!("Bearer {}", tokens_vec[i + 1]),
                    );
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--connect-timeout" => {
                if i + 1 < tokens_vec.len() {
                    if let Ok(timeout) = tokens_vec[i + 1].parse::<u32>() {
                        curl_data.connect_timeout = Some(timeout);
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-m" | "--max-time" => {
                if i + 1 < tokens_vec.len() {
                    if let Ok(timeout) = tokens_vec[i + 1].parse::<u32>() {
                        curl_data.max_time = Some(timeout);
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-L" | "--location" => {
                curl_data.location = true;
                i += 1;
            }
            "--max-redirs" => {
                if i + 1 < tokens_vec.len() {
                    if let Ok(redirs) = tokens_vec[i + 1].parse::<u32>() {
                        curl_data.max_redirs = Some(redirs);
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--retry" => {
                if i + 1 < tokens_vec.len() {
                    if let Ok(retry) = tokens_vec[i + 1].parse::<u32>() {
                        curl_data.retry = Some(retry);
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--no-alpn" => {
                curl_data.no_alpn = true;
                i += 1;
            }
            "--cacert" => {
                if i + 1 < tokens_vec.len() {
                    curl_data.ssl_options.cacert = Some(tokens_vec[i + 1].to_string());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--cert" => {
                if i + 1 < tokens_vec.len() {
                    curl_data.ssl_options.cert = Some(tokens_vec[i + 1].to_string());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--key" => {
                if i + 1 < tokens_vec.len() {
                    curl_data.ssl_options.key = Some(tokens_vec[i + 1].to_string());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--cert-type" => {
                if i + 1 < tokens_vec.len() {
                    curl_data.ssl_options.cert_type = Some(tokens_vec[i + 1].to_string());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--key-type" => {
                if i + 1 < tokens_vec.len() {
                    curl_data.ssl_options.key_type = Some(tokens_vec[i + 1].to_string());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-o" | "--output" => {
                if i + 1 < tokens_vec.len() {
                    curl_data.output = Some(tokens_vec[i + 1].to_string());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-w" | "--write-out" => {
                if i + 1 < tokens_vec.len() {
                    curl_data.write_out = Some(tokens_vec[i + 1].to_string());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "-s" | "--silent" => {
                
                i += 1;
            }
            _ => {
                
                if !token.starts_with('-') && curl_data.url.is_empty() {
                    
                    let _ = Url::parse(token)?;
                    curl_data.url = token.to_string();
                    i += 1;
                } else {
                    
                    i += 1;
                    if i < tokens_vec.len() && !tokens_vec[i].starts_with('-') {
                        i += 1;
                    }
                }
            }
        }
    }

    
    if curl_data.method == "GET"
        && (!curl_data.url_encoded_params.is_empty() || curl_data.data.is_some())
    {
        
        let mut url = Url::parse(&curl_data.url)?;

        
        for (key, value) in &curl_data.url_encoded_params {
            url.query_pairs_mut().append_pair(key, value);
        }

        
        if let Some(data) = &curl_data.data {
            if data.contains('=') {
                for param in data.split('&') {
                    let parts: Vec<&str> = param.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        url.query_pairs_mut().append_pair(parts[0], parts[1]);
                    }
                }
            }
        }

        
        curl_data.url = url.to_string();

        
        curl_data.data = None;
    }

    
    if curl_data.url.is_empty() {
        return Err(ParseError::MissingUrl);
    }

    Ok(curl_data)
}
