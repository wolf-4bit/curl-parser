use crate::converters::{utils, Convert};
use crate::CurlCommand;

pub struct Axios;

impl Convert for Axios {
    fn convert(&self, curl_cmd: &CurlCommand) -> String {
        let mut output = String::new();

        let mut imports = String::from("const axios = require('axios');\n");

        if curl_cmd.binary_file.is_some() || curl_cmd.output.is_some() {
            imports.push_str("const fs = require('fs');\n");
        }

        output.push_str(&imports);
        output.push_str("\n");

        if let Some(binary_file) = &curl_cmd.binary_file {
            output.push_str(&format!(
                "// Read binary file\nconst binaryData = fs.readFileSync('{}');\n\n",
                utils::escape_single_quotes(binary_file)
            ));
        }

        output.push_str("const config = {\n");

        output.push_str(&format!(
            "  method: '{}',\n",
            curl_cmd.method.to_lowercase()
        ));

        output.push_str(&format!("  url: '{}',\n", curl_cmd.url));

        let mut has_content_type = false;
        if !curl_cmd.headers.is_empty()
            || curl_cmd.user_agent.is_some()
            || curl_cmd.binary_file.is_some()
        {
            output.push_str("  headers: {\n");

            for (key, value) in &curl_cmd.headers {
                output.push_str(&format!(
                    "    '{}': '{}',\n",
                    key,
                    utils::escape_single_quotes(value)
                ));
                if key.to_lowercase() == "content-type" {
                    has_content_type = true;
                }
            }

            if let Some(user_agent) = &curl_cmd.user_agent {
                output.push_str(&format!(
                    "    'User-Agent': '{}',\n",
                    utils::escape_single_quotes(user_agent)
                ));
            }

            if curl_cmd.binary_file.is_some() && !has_content_type {
                output.push_str("    'Content-Type': 'application/octet-stream',\n");
            }

            output.push_str("  },\n");
        } else if curl_cmd.binary_file.is_some() {
            output.push_str("  headers: {\n");
            output.push_str("    'Content-Type': 'application/octet-stream',\n");
            output.push_str("  },\n");
        }

        if let Some(data) = &curl_cmd.data {
            if data.starts_with('{') && data.ends_with('}') {
                output.push_str(&format!("  data: {},\n", data));
            } else {
                output.push_str(&format!(
                    "  data: '{}',\n",
                    utils::escape_single_quotes(data)
                ));
            }
        } else if curl_cmd.binary_file.is_some() {
            output.push_str("  data: binaryData,\n");
        }

        if !curl_cmd.form.is_empty() {
            output.push_str("  formData: {\n");

            for (key, value) in &curl_cmd.form {
                output.push_str(&format!(
                    "    '{}': '{}',\n",
                    key,
                    utils::escape_single_quotes(value)
                ));
            }

            output.push_str("  },\n");
        }

        if let Some((username, password)) = &curl_cmd.auth {
            output.push_str("  auth: {\n");
            output.push_str(&format!(
                "    username: '{}',\n",
                utils::escape_single_quotes(username)
            ));
            output.push_str(&format!(
                "    password: '{}',\n",
                utils::escape_single_quotes(password)
            ));
            output.push_str("  },\n");
        }

        if curl_cmd.insecure {
            output.push_str(
                "  httpsAgent: new (require('https').Agent)({ rejectUnauthorized: false }),\n",
            );
        }

        if let Some(proxy) = &curl_cmd.proxy {
            output.push_str(&format!(
                "  proxy: {{\n    host: '{}',\n    protocol: 'http'\n  }},\n",
                utils::escape_single_quotes(proxy)
            ));
        }

        let mut has_timeout_config = false;

        if curl_cmd.connect_timeout.is_some() || curl_cmd.max_time.is_some() {
            output.push_str("  timeout: {\n");
            has_timeout_config = true;

            if let Some(timeout) = curl_cmd.connect_timeout {
                output.push_str(&format!("    connect: {},\n", timeout));
            }

            if let Some(timeout) = curl_cmd.max_time {
                output.push_str(&format!("    request: {},\n", timeout));
            }

            output.push_str("  },\n");
        }

        if curl_cmd.location {
            if !has_timeout_config {
                output.push_str("  maxRedirects: ");

                if let Some(max_redirs) = curl_cmd.max_redirs {
                    output.push_str(&format!("{}", max_redirs));
                } else {
                    output.push_str("5"); 
                }

                output.push_str(",\n");
            } else {
                if let Some(max_redirs) = curl_cmd.max_redirs {
                    output.push_str(&format!("  maxRedirects: {},\n", max_redirs));
                } else {
                    output.push_str("  maxRedirects: 5,\n");
                }
            }
        } else {
            output.push_str("  maxRedirects: 0,\n");
        }

        output.push_str("};\n\n");

        output.push_str("axios(config)\n");
        output.push_str("  .then(response => {\n");

        if let Some(output_file) = &curl_cmd.output {
            output.push_str(&format!(
                "    // Save response to file\n    fs.writeFileSync('{}', response.data);\n",
                utils::escape_single_quotes(output_file)
            ));
            output.push_str(&format!(
                "    console.log(`Response saved to {}`);\n",
                utils::escape_single_quotes(output_file)
            ));
        } else {
            output.push_str("    console.log(JSON.stringify(response.data));\n");
        }

        output.push_str("  })\n");
        output.push_str("  .catch(error => {\n");
        output.push_str("    console.error(error);\n");
        output.push_str("  });\n");

        output
    }
}
