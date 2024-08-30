use std::{collections::HashMap, str::FromStr};

use digest::{Digest, DynDigest};

#[derive(Debug, thiserror::Error)]
pub enum ChecksumError {
    #[error("Unknown checksum algortihm")]
    UnknownChecksumAlgorithm,
    #[error("Invalid checksum format: {0}")]
    ChecksumFormat(String),
    #[error("Invalid checksum: got {0} expected {1}")]
    InvalidChecksum(String, String),
}

#[derive(Debug)]
pub enum ChecksumAlgorithm {
    MD5,
    SHA1,
    SHA256,
    SHA512,
}

impl ChecksumAlgorithm {
    fn infer(value: &str) -> Option<Self> {
        match value.len() {
            32 => Some(ChecksumAlgorithm::MD5),
            40 => Some(ChecksumAlgorithm::SHA1),
            64 => Some(ChecksumAlgorithm::SHA256),
            128 => Some(ChecksumAlgorithm::SHA512),
            _ => None,
        }
    }

    fn into_digest(self) -> Box<dyn DynDigest> {
        match self {
            ChecksumAlgorithm::MD5 => Box::new(md5::Md5::new()),
            ChecksumAlgorithm::SHA1 => Box::new(sha1::Sha1::new()),
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

impl FromStr for Checksum {
    type Err = ChecksumError; // TODO: Implement proper error handling

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut files = HashMap::new();
        let mut algorithm = None;

        for line in s.lines() {
            let mut parts = line.splitn(2, ' ');
            let (hash, filename) = parts
                .next()
                .zip(parts.next())
                .ok_or(ChecksumError::ChecksumFormat(line.to_owned()))?;

            algorithm = ChecksumAlgorithm::infer(hash);
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
