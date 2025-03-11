use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Missing URL in curl command")]
    MissingUrl,

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Failed to parse curl command: {0}")]
    ParseFailure(String),
}
