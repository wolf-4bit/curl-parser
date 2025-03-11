use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod converters;
pub mod parser;


pub use parser::{parse_curl_command, ParseError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversionFormat {
    PythonRequests,
    NodeJS,
}

impl std::str::FromStr for ConversionFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "python" | "python-requests" | "requests" => Ok(ConversionFormat::PythonRequests),
            "node" | "nodejs" | "javascript" | "js" => Ok(ConversionFormat::NodeJS),
            _ => Err(format!("Unknown conversion format: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurlCommand {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub data: Option<String>,
    pub data_is_json: bool,
    pub binary_file: Option<String>,
    pub form: HashMap<String, String>,
    pub auth: Option<(String, String)>,
    pub oauth2_bearer: Option<String>,
    pub cookies: HashMap<String, String>,
    pub user_agent: Option<String>,
    pub insecure: bool,
    pub compressed: bool,
    pub proxy: Option<String>,
    pub proxy_auth: Option<(String, String)>,
    pub connect_timeout: Option<u32>,
    pub max_time: Option<u32>,
    pub location: bool,
    pub max_redirs: Option<u32>,
    pub output: Option<String>,
    pub auth_type: Option<AuthType>,
    pub url_encoded_params: HashMap<String, String>,
    pub files: HashMap<String, FileUpload>,
    pub ssl_options: SslOptions,
    pub write_out: Option<String>,
    pub retry: Option<u32>,
    pub no_alpn: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthType {
    Basic,
    Digest,
    Ntlm,
    Negotiate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUpload {
    pub path: String,
    pub content_type: Option<String>,
    pub filename: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SslOptions {
    pub cacert: Option<String>,
    pub cert: Option<String>,
    pub key: Option<String>,
    pub cert_type: Option<String>,
    pub key_type: Option<String>,
}

impl Default for CurlCommand {
    fn default() -> Self {
        Self {
            method: "GET".to_string(),
            url: String::new(),
            headers: HashMap::new(),
            data: None,
            data_is_json: false,
            binary_file: None,
            form: HashMap::new(),
            auth: None,
            oauth2_bearer: None,
            cookies: HashMap::new(),
            user_agent: None,
            insecure: false,
            compressed: false,
            proxy: None,
            proxy_auth: None,
            connect_timeout: None,
            max_time: None,
            location: false,
            max_redirs: None,
            output: None,
            auth_type: None,
            url_encoded_params: HashMap::new(),
            files: HashMap::new(),
            ssl_options: SslOptions::default(),
            write_out: None,
            retry: None,
            no_alpn: false,
        }
    }
}
