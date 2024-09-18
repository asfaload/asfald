use std::{collections::HashMap, path::Path, str::FromStr};

use digest::{Digest, DynDigest};

#[derive(Debug, thiserror::Error)]
pub enum ChecksumError {
    #[error("Unknown checksum algortihm")]
    UnknownChecksumAlgorithm,
    #[error("Invalid checksum format: {0}")]
    ChecksumFormat(String),
    #[error("Invalid filename: {0}")]
    FileNamePart(String),
    #[error("Invalid checksum: got {0} expected {1}")]
    InvalidChecksum(String, String),
}

#[derive(Debug)]
pub enum ChecksumAlgorithm {
    SHA256,
    SHA512,
}

impl ChecksumAlgorithm {
    fn infer(value: &str) -> Option<Self> {
        match value.len() {
            64 => Some(ChecksumAlgorithm::SHA256),
            128 => Some(ChecksumAlgorithm::SHA512),
            _ => None,
        }
    }

    fn into_digest(self) -> Box<dyn DynDigest> {
        match self {
            ChecksumAlgorithm::SHA256 => Box::new(sha2::Sha256::new()),
            ChecksumAlgorithm::SHA512 => Box::new(sha2::Sha512::new()),
        }
    }
}

#[derive(Debug)]
pub struct Checksum {
    algorithm: ChecksumAlgorithm,
    files: HashMap<String, String>,
}

impl Checksum {
    pub fn into_validator(self, file: &str) -> Option<ChecksumValidator> {
        self.files
            .get(file)
            .map(|hash| ChecksumValidator::new(self.algorithm, hash))
    }
}

fn handle_file_path(filename: &str) -> Result<&str, ChecksumError> {
    // Manipulate the filename to ignore its path as we do not use it anyway
    // and it causes us trouble if left in
    let filepath = Path::new(filename);
    filepath
        // Ignore path component of the filename
        .file_name()
        .and_then(|p| p.to_str())
        .ok_or(ChecksumError::FileNamePart(filename.to_string()))
}

impl FromStr for Checksum {
    type Err = ChecksumError; // TODO: Implement proper error handling

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut files = HashMap::new();
        let mut algorithm = None;

        for line in s.lines() {
            let mut parts = line.splitn(2, ' ');
            let (hash, filename) = parts
                .next()
                .zip(parts.next().map(|s| {
                    // Binary files are prefixed by a `*`, which is not part of the filename
                    // We remove this prefix from the extracted filename.
                    if s.starts_with("*") {
                        // Cannot return s, the local variable of type &str (ERR E0515), so we return s string here
                        // and convert to string in the else
                        s.replacen("*", "", 1)
                    } else {
                        s.to_string()
                    }
                }))
                .ok_or(ChecksumError::ChecksumFormat(line.to_owned()))?;

            algorithm = ChecksumAlgorithm::infer(hash);
            let filename = handle_file_path(filename.as_str())?;
            files.insert(filename.trim().to_owned(), hash.trim().to_owned());
        }

        let algorithm = algorithm.ok_or(ChecksumError::UnknownChecksumAlgorithm)?;
        Ok(Checksum { algorithm, files })
    }
}

pub struct ChecksumValidator {
    hash: String,
    digest: Box<dyn DynDigest>,
}

impl ChecksumValidator {
    fn new(algo: ChecksumAlgorithm, hash: &str) -> Self {
        ChecksumValidator {
            hash: hash.to_owned(),
            digest: algo.into_digest(),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.digest.update(data);
    }

    pub fn validate(self) -> Result<(), ChecksumError> {
        let hash = self.digest.finalize();
        let hash = hex::encode(hash);

        (hash.to_lowercase() == self.hash.to_lowercase())
            .then_some(())
            .ok_or(ChecksumError::InvalidChecksum(hash, self.hash))
    }
}

#[cfg(test)]
mod checksum_helpers_tests {
    use super::*;

    #[test]
    // Do we remove all path components from filenames in the checksums file?
    fn test_handle_file_path() {
        let filename = "./my_file.txt";
        let r = handle_file_path(filename);
        assert!(r.is_ok());
        assert_eq!("my_file.txt", r.unwrap());

        let filename = "/path/to/file/my_file.txt";
        let r = handle_file_path(filename);
        assert!(r.is_ok());
        assert_eq!("my_file.txt", r.unwrap());
    }
}
