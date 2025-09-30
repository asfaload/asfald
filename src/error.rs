use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("URL parsing failed: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Hash computation failed: {0}")]
    HashError(String),

    #[error("Asset not found in release: {0}")]
    AssetNotFound(String),

    #[error("Hash verification failed: expected {expected}, got {actual}")]
    HashVerificationFailed { expected: String, actual: String },

    #[error("Unsupported hash algorithm: {0}")]
    UnsupportedAlgorithm(String),

    #[error("Invalid URL format: {0}")]
    InvalidUrlFormat(String),

    #[error("GitHub API error: {0}")]
    GitHubApiError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
