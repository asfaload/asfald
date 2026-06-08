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

    #[error(transparent)]
    ClientLib(#[from] client_lib::ClientLibError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use client_lib::ClientLibError;

    #[test]
    fn client_lib_error_converts_into_asfald_error() {
        let lib_err = ClientLibError::HttpError {
            status: 503,
            url: "https://backend.example".to_string(),
        };
        let err: Error = lib_err.into();
        assert!(matches!(err, Error::ClientLib(_)));
    }
}
