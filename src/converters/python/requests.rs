use crate::converters::{utils, Convert};
use crate::AuthType;
use crate::CurlCommand;

pub struct Requests;

impl Convert for Requests {
    fn convert(&self, curl_cmd: &CurlCommand) -> String {
        let mut output = String::new();

        output.push_str("import requests\n\n");

        if !curl_cmd.headers.is_empty()
            || curl_cmd.user_agent.is_some()
            || curl_cmd.oauth2_bearer.is_some()
        {
            output.push_str("headers = {\n");

            for (key, value) in &curl_cmd.headers {
                if key != "Authorization" || curl_cmd.oauth2_bearer.is_none() {
                    output.push_str(&format!(
                        "    \"{}\": \"{}\",\n",
                        key,
                        utils::escape_quotes(value)
                    ));
                }
            }

            if let Some(user_agent) = &curl_cmd.user_agent {
                output.push_str(&format!(
                    "    \"User-Agent\": \"{}\",\n",
                    utils::escape_quotes(user_agent)
                ));
            }

            if let Some(token) = &curl_cmd.oauth2_bearer {
                output.push_str(&format!(
                    "    \"Authorization\": \"Bearer {}\",\n",
                    utils::escape_quotes(token)
                ));
            }

            output.push_str("}\n\n");
        }

        if !curl_cmd.cookies.is_empty() {
            output.push_str("cookies = {\n");

            for (key, value) in &curl_cmd.cookies {
                output.push_str(&format!(
                    "    \"{}\": \"{}\",\n",
                    key,
                    utils::escape_quotes(value)
                ));
            }

            output.push_str("}\n\n");
        }

        if let Some(data) = &curl_cmd.data {
            if curl_cmd.data_is_json || (data.starts_with('{') && data.ends_with('}')) {
                output.push_str("import json\n");

                if curl_cmd.data_is_json {
                    output.push_str(&format!("json_data = {}\n\n", data));
                } else {
                    output.push_str(&format!(
                        "json_data = json.loads('{}')\n\n",
                        utils::escape_quotes(data)
                    ));
                }
            } else {
                output.push_str(&format!("data = \"{}\"\n\n", utils::escape_quotes(data)));
            }
        } else if let Some(binary_file) = &curl_cmd.binary_file {
            output.push_str("# Binary file upload\n");
            output.push_str(&format!(
                "with open(\"{}\", \"rb\") as f:\n",
                utils::escape_quotes(binary_file)
            ));
            output.push_str("    binary_data = f.read()\n\n");
        }
        if !curl_cmd.url_encoded_params.is_empty() {
            output.push_str("params = {\n");

            for (key, value) in &curl_cmd.url_encoded_params {
                output.push_str(&format!(
                    "    \"{}\": \"{}\",\n",
                    key,
                    utils::escape_quotes(value)
                ));
            }

            output.push_str("}\n\n");
        }

        if !curl_cmd.form.is_empty() || !curl_cmd.files.is_empty() {
            output.push_str("files = {\n");

            for (key, value) in &curl_cmd.form {
                output.push_str(&format!(
                    "    \"{}\": \"{}\",\n",
                    key,
                    utils::escape_quotes(value)
                ));
            }

            for (key, file_upload) in &curl_cmd.files {
                if let Some(content_type) = &file_upload.content_type {
                    
                    let filename = match &file_upload.filename {
                        Some(name) => name.clone(),
                        None => {
                            
                            let path_parts: Vec<&str> = file_upload.path.split('/').collect();
                            path_parts.last().unwrap_or(&"file").to_string()
                        }
                    };

                    output.push_str(&format!(
                        "    \"{}\": (\"{}\" , open(\"{}\", \"rb\"), \"{}\"),\n",
                        key,
                        utils::escape_quotes(&filename),
                        utils::escape_quotes(&file_upload.path),
                        utils::escape_quotes(content_type)
                    ));
                } else {
                    
                    output.push_str(&format!(
                        "    \"{}\": open(\"{}\", \"rb\"),\n",
                        key,
                        utils::escape_quotes(&file_upload.path)
                    ));
                }
            }

            output.push_str("}\n\n");
        }

        if let Some((username, password)) = &curl_cmd.auth {
            match curl_cmd.auth_type {
                Some(AuthType::Digest) => {
                    output.push_str("from requests.auth import HTTPDigestAuth\n");
                    output.push_str(&format!(
                        "auth = HTTPDigestAuth(\"{}\", \"{}\")\n\n",
                        utils::escape_quotes(username),
                        utils::escape_quotes(password)
                    ));
                }
                Some(AuthType::Ntlm) => {
                    output.push_str("# Note: NTLM auth requires requests_ntlm package\n");
                    output.push_str("from requests_ntlm import HttpNtlmAuth\n");
                    output.push_str(&format!(
                        "auth = HttpNtlmAuth(\"{}\", \"{}\")\n\n",
                        utils::escape_quotes(username),
                        utils::escape_quotes(password)
                    ));
                }
                Some(AuthType::Negotiate) => {
                    output.push_str(
                        "# Note: Negotiate/Kerberos auth requires requests_kerberos package\n",
                    );
                    output.push_str("from requests_kerberos import HTTPKerberosAuth\n");
                    output.push_str("auth = HTTPKerberosAuth()\n\n");
                }
                _ => {
                    output.push_str(&format!(
                        "auth = (\"{}\", \"{}\")\n\n",
                        utils::escape_quotes(username),
                        utils::escape_quotes(password)
                    ));
                }
            }
        }

        if curl_cmd.ssl_options.cacert.is_some()
            || curl_cmd.ssl_options.cert.is_some()
            || curl_cmd.ssl_options.key.is_some()
        {
            if let Some(cert) = &curl_cmd.ssl_options.cert {
                if let Some(key) = &curl_cmd.ssl_options.key {
                    output.push_str(&format!(
                        "cert = (\"{}\", \"{}\")\n\n",
                        utils::escape_quotes(cert),
                        utils::escape_quotes(key)
                    ));
                } else {
                    output.push_str(&format!("cert = \"{}\"\n\n", utils::escape_quotes(cert)));
                }
            }

            if let Some(cacert) = &curl_cmd.ssl_options.cacert {
                output.push_str(&format!(
                    "verify = \"{}\"\n\n",
                    utils::escape_quotes(cacert)
                ));
            }
        }

        if let Some(proxy) = &curl_cmd.proxy {
            let proxy_str = proxy.clone();

            if let Some((username, password)) = &curl_cmd.proxy_auth {
                
                let proxy_parts: Vec<&str> = proxy_str.split("://").collect();
                let protocol = if proxy_parts.len() > 1 {
                    proxy_parts[0]
                } else {
                    "http"
                };
                let host = if proxy_parts.len() > 1 {
                    proxy_parts[1]
                } else {
                    proxy_parts[0]
                };

                output.push_str(&format!("proxies = {{\n    'http': '{}://{}:{}@{}',\n    'https': '{}://{}:{}@{}'\n}}\n\n", 
                    protocol, utils::escape_quotes(username), utils::escape_quotes(password), host,
                    protocol, utils::escape_quotes(username), utils::escape_quotes(password), host));
            } else {
                output.push_str(&format!(
                    "proxies = {{\n    'http': '{}',\n    'https': '{}'\n}}\n\n",
                    proxy_str, proxy_str
                ));
            }
        }

        let mut params = Vec::new();
        let mut param_strings = Vec::new();

        if !curl_cmd.headers.is_empty()
            || curl_cmd.user_agent.is_some()
            || curl_cmd.oauth2_bearer.is_some()
        {
            params.push("headers=headers");
        }

        if !curl_cmd.cookies.is_empty() {
            params.push("cookies=cookies");
        }

        if let Some(data) = &curl_cmd.data {
            if curl_cmd.data_is_json || (data.starts_with('{') && data.ends_with('}')) {
                params.push("json=json_data");
            } else {
                params.push("data=data");
            }
        } else if let Some(_) = &curl_cmd.binary_file {
            params.push("data=binary_data");
        }

        if !curl_cmd.url_encoded_params.is_empty() {
            params.push("params=params");
        }

        if !curl_cmd.form.is_empty() || !curl_cmd.files.is_empty() {
            params.push("files=files");
        }

        if let Some(_) = &curl_cmd.auth {
            params.push("auth=auth");
        }

        if let Some(_cert) = &curl_cmd.ssl_options.cert {
            params.push("cert=cert");
        }

        if curl_cmd.insecure {
            params.push("verify=False");
        } else if let Some(_) = &curl_cmd.ssl_options.cacert {
            params.push("verify=verify");
        }

        if let Some(_) = &curl_cmd.proxy {
            params.push("proxies=proxies");
        }

        if let Some(timeout) = curl_cmd.connect_timeout {
            let timeout_secs = timeout as f32 / 1000.0;
            let timeout_param = format!("timeout={}", timeout_secs);
            param_strings.push(timeout_param);
            params.push(param_strings.last().unwrap());
        } else if let Some(timeout) = curl_cmd.max_time {
            let timeout_secs = timeout as f32 / 1000.0;
            let timeout_param = format!("timeout={}", timeout_secs);
            param_strings.push(timeout_param);
            params.push(param_strings.last().unwrap());
        }

        if let Some(retry) = curl_cmd.retry {
            output.push_str("from requests.adapters import HTTPAdapter\n");
            output.push_str("from urllib3.util.retry import Retry\n\n");
            output.push_str(&format!(
                "retry_strategy = Retry(total={}, backoff_factor=1)\n",
                retry
            ));
            output.push_str("adapter = HTTPAdapter(max_retries=retry_strategy)\n");
            output.push_str("session = requests.Session()\n");
            output.push_str("session.mount('http://', adapter)\n");
            output.push_str("session.mount('https://', adapter)\n\n");
        }

        if curl_cmd.location {
            params.push("allow_redirects=True");

            if let Some(max_redirs) = curl_cmd.max_redirs {
                
                output.push_str(&format!("\n# Custom session for redirect control\n"));

                if curl_cmd.retry.is_none() {
                    
                    output.push_str("session = requests.Session()\n");
                }

                output.push_str(&format!("session.max_redirects = {}\n", max_redirs));

                
                output.push_str(&format!(
                    "response = session.{}(\"{}\", ",
                    curl_cmd.method.to_lowercase(),
                    curl_cmd.url
                ));

                
                if !params.is_empty() {
                    output.push_str(&params.join(", "));
                }

                output.push_str(")\n");

                
                if let Some(output_file) = &curl_cmd.output {
                    output.push_str(&format!("\n# Save response content to file\nwith open(\"{}\", \"wb\") as f:\n    f.write(response.content)\n", utils::escape_quotes(output_file)));
                } else {
                    
                    if let Some(write_out) = &curl_cmd.write_out {
                        self.handle_write_out(&mut output, write_out);
                    } else {
                        output.push_str("\nprint(response.text)\n");
                    }
                }

                return output;
            }
        } else {
            params.push("allow_redirects=False");
        }

        
        if curl_cmd.retry.is_some() {
            
            let method_lower = curl_cmd.method.to_lowercase();

            
            let is_standard_method = matches!(
                method_lower.as_str(),
                "get" | "post" | "put" | "delete" | "head" | "options" | "patch"
            );

            if is_standard_method {
                
                output.push_str(&format!(
                    "response = session.{}(\"{}\", ",
                    method_lower, curl_cmd.url
                ));
            } else {
                
                output.push_str(&format!(
                    "response = session.request(\"{}\", \"{}\", ",
                    method_lower, curl_cmd.url
                ));
            }
        } else {
            
            let method_lower = curl_cmd.method.to_lowercase();

            
            let is_standard_method = matches!(
                method_lower.as_str(),
                "get" | "post" | "put" | "delete" | "head" | "options" | "patch"
            );

            if is_standard_method {
                
                output.push_str(&format!(
                    "response = requests.{}(\"{}\", ",
                    method_lower, curl_cmd.url
                ));
            } else {
                
                output.push_str(&format!(
                    "response = requests.request(\"{}\", \"{}\", ",
                    method_lower, curl_cmd.url
                ));
            }
        }

        
        if !params.is_empty() {
            output.push_str(&params.join(", "));
        }

        output.push_str(")\n");

        
        if let Some(output_file) = &curl_cmd.output {
            output.push_str(&format!("\n# Save response content to file\nwith open(\"{}\", \"wb\") as f:\n    f.write(response.content)\n", utils::escape_quotes(output_file)));
        } else {
            
            if let Some(write_out) = &curl_cmd.write_out {
                self.handle_write_out(&mut output, write_out);
            } else {
                
                output.push_str("\nprint(response.text)\n");
            }
        }

        output
    }
}

impl Requests {
    
    fn handle_write_out(&self, output: &mut String, format: &str) {
        if format.contains("%{http_code}") {
            output.push_str("\n# Print status code\nprint(response.status_code)\n");
        } else if format.contains("%{time_total}") {
            output
                .push_str("\n# To calculate time_total, add this code to the top of the script:\n");
            output.push_str("import time\nstart_time = time.time()\n\n");
            output.push_str("# Print time total at the end\nprint(f\"Time: {time.time() - start_time:.6f} seconds\")\n");
        } else if format.contains("%{size_download}") {
            output.push_str("\n# Print content size\nprint(len(response.content))\n");
        } else if format.contains("%{content_type}") {
            output.push_str(
                "\n# Print content type\nprint(response.headers.get('Content-Type', ''))\n",
            );
        } else {
            
            output.push_str(
                "\n# Using default output since write-out format contains unsupported specifiers\n",
            );
            output.push_str("print(response.text)\n");
        }
    }
}
