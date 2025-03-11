use crate::parser::error::ParseError;


pub fn tokenize_command(command: &str) -> Result<Vec<String>, ParseError> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escape_next = false;

    for c in command.chars() {
        if escape_next {
            current_token.push(c);
            escape_next = false;
        } else if c == '\\' {
            escape_next = true;
        } else if c == '\'' && !in_double_quote {
            in_single_quote = !in_single_quote;
        } else if c == '"' && !in_single_quote {
            in_double_quote = !in_double_quote;
        } else if c.is_whitespace() && !in_single_quote && !in_double_quote {
            if !current_token.is_empty() {
                tokens.push(current_token);
                current_token = String::new();
            }
        } else {
            current_token.push(c);
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    
    if in_single_quote || in_double_quote {
        return Err(ParseError::ParseFailure(
            "Unclosed quotes in command".to_string(),
        ));
    }

    Ok(tokens)
}
