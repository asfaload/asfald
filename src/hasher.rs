use crate::{Error, Result};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HashAlgorithm {
    Sha256,
}

impl std::str::FromStr for HashAlgorithm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "sha256" => Ok(HashAlgorithm::Sha256),
            _ => Err(Error::UnsupportedAlgorithm(s.to_string())),
        }
    }
}

impl std::fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashAlgorithm::Sha256 => write!(f, "sha256"),
        }
    }
}

pub struct Hasher;

impl Hasher {
    pub fn compute_file_hash<P: AsRef<Path>>(path: P, algorithm: &HashAlgorithm) -> Result<String> {
        let mut file = std::fs::File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        Self::compute_hash(&buffer, algorithm)
    }

    pub fn compute_hash(data: &[u8], algorithm: &HashAlgorithm) -> Result<String> {
        let hash = match algorithm {
            HashAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize()
            }
        };

        Ok(hex::encode(hash))
    }

    pub fn parse_digest(digest: &str) -> Result<(HashAlgorithm, String)> {
        let parts: Vec<&str> = digest.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(Error::HashError(format!(
                "Invalid digest format: {}",
                digest
            )));
        }

        let algorithm = parts[0].parse()?;
        let hash_value = parts[1].to_string();

        Ok((algorithm, hash_value))
    }
}
