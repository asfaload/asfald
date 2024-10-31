mod v1 {
    use ::serde::{Deserialize, Serialize};
    use chrono::{serde, DateTime};

    #[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
    pub enum Algo {
        Md5,
        Sha1,
        Sha256,
        Sha512,
    }
    #[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
    pub struct FileChecksum {
        pub fileName: String,
        pub algo: Algo,
        pub source: String,
        pub hash: String,
    }
    #[derive(Serialize, Deserialize)]
    pub struct AsfaloadIndex {
        pub mirroredOn: DateTime<chrono::Utc>,
        pub publishedOn: DateTime<chrono::Utc>,
        pub version: i32,
        pub publishedFiles: Vec<FileChecksum>,
    }

    #[derive(PartialEq, Debug)]
    pub enum ChecksumError {
        NotFound,
        MultipleValues,
    }

    impl AsfaloadIndex {
        pub fn get_hash_for_file(
            self,
            filename: &str,
            algo: Algo,
        ) -> Result<FileChecksum, ChecksumError> {
            let found: Vec<FileChecksum> = self
                .publishedFiles
                .into_iter()
                .filter(|file| file.fileName == filename && file.algo == algo)
                .collect();
            match found.len() {
                1 => Ok(found[0].clone()),
                0 => Err(ChecksumError::NotFound),
                _ => {
                    let first_hash = found[0].hash.clone();
                    if found.iter().all(|file| file.hash == first_hash) {
                        // We found multiple hash values, but arbitrarily use the first one
                        Ok(found[0].clone())
                    } else {
                        Err(ChecksumError::MultipleValues)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod lib_tests {
    use v1::{AsfaloadIndex, ChecksumError};

    use super::*;
    use anyhow::Result;
    use chrono::DateTime;

    #[test]
    fn parse_json() -> Result<()> {
        let index: AsfaloadIndex = serde_json::from_str(JSON)?;
        assert_eq!(index.version, 1);
        assert_eq!(
            index.mirroredOn,
            serde_json::from_str::<DateTime<chrono::Utc>>("\"2024-10-30T10:48:24.9397405+00:00\"")?
        );
        Ok(())
    }

    #[test]
    fn get_file_hash() -> Result<()> {
        let index: AsfaloadIndex = serde_json::from_str(JSON)?;

        // Normal situation: one hash is found
        let file_entry = index.get_hash_for_file("hctl_freebsd_arm64.tar.gz", v1::Algo::Sha256);
        assert_eq!(
            file_entry.map(|f| f.hash),
            Ok("03ecde4a2efdbfa234b6aaa3ab166ee92e83ffd0d3521b455b51d00ff171909b".to_string())
        );

        // Two entries with the same hash values are found, should work fine
        // FIXME: best solution to avoid redefining index as workaround for borrow checker
        // complaint?
        let index: AsfaloadIndex = serde_json::from_str(JSON)?;
        let file_entry = index.get_hash_for_file("hctl_darwin_arm64.tar.gz", v1::Algo::Sha256);
        assert_eq!(
            file_entry.map(|f| f.hash),
            Ok("e9e40eeb6c6c049c863cdf8769a8a9553c3739bac5ab1e05444509d676185e6e".to_string())
        );
        // Two entries with the same hash values are found, should work fine
        // FIXME: best solution to avoid redefining index as workaround for borrow checker
        // complaint?
        let index: AsfaloadIndex = serde_json::from_str(JSON)?;
        let file_entry = index.get_hash_for_file("hctl_darwin_x86_64.tar.gz", v1::Algo::Sha256);
        assert_eq!(file_entry, Err(ChecksumError::MultipleValues));

        Ok(())
    }
    // This json tweaked to include specific situations:
    // - sha is duplicated in 2 checksums files, with the same value (we can use this)
    // - sha is duplicated in 2 checksums files but with different values (we cannot determine
    // which is right)
    const JSON: &str = r#"

        {
        "mirroredOn": "2024-10-30T10:48:24.9397405+00:00",
        "publishedOn": "2024-10-30T10:48:24.9397986+00:00",
        "version": 1,
        "publishedFiles": [
            {
            "fileName": "hctl_darwin_arm64.tar.gz",
            "algo": "Sha256",
            "source": "hctl_0.3.1_checksums.txt",
            "hash": "e9e40eeb6c6c049c863cdf8769a8a9553c3739bac5ab1e05444509d676185e6e"
            },
            {
            "fileName": "hctl_darwin_arm64.tar.gz",
            "algo": "Sha256",
            "source": "hctl_0.3.1_checksums.duplicate.txt",
            "hash": "e9e40eeb6c6c049c863cdf8769a8a9553c3739bac5ab1e05444509d676185e6e"
            },
            {
            "fileName": "hctl_darwin_x86_64.tar.gz",
            "algo": "Sha256",
            "source": "hctl_0.3.1_checksums.txt",
            "hash": "2bb9254023af4307db99e1f0165e481e54f78e4cf23fa1f169a229ffcc539789"
            },
            {
            "fileName": "hctl_darwin_x86_64.tar.gz",
            "algo": "Sha256",
            "source": "hctl_0.3.1_checksums.invalid_duplicate.txt",
            "hash": "0000000023af4307db99e1f0165e481e54f78e4cf23fa1f169a229ffcc539789"
            },
            {
            "fileName": "hctl_freebsd_arm64.tar.gz",
            "algo": "Sha256",
            "source": "hctl_0.3.1_checksums.txt",
            "hash": "03ecde4a2efdbfa234b6aaa3ab166ee92e83ffd0d3521b455b51d00ff171909b"
            },
            {
            "fileName": "hctl_freebsd_i386.tar.gz",
            "algo": "Sha256",
            "source": "hctl_0.3.1_checksums.txt",
            "hash": "d16af5a91631f0c2232c747fa773a8dab21aa896894bbba55847e74a100eec9f"
            },
            {
            "fileName": "hctl_freebsd_x86_64.tar.gz",
            "algo": "Sha256",
            "source": "hctl_0.3.1_checksums.txt",
            "hash": "b2dc13f4468e8ebf50c8bfe0634508f7a77d3e7b24121004638d71818a283b78"
            },
            {
            "fileName": "hctl_linux_arm64.tar.gz",
            "algo": "Sha256",
            "source": "hctl_0.3.1_checksums.txt",
            "hash": "1c737052a44b969217445a0f2c9c31d59fd3a2f992db07dc66fe8810a26f8d75"
            },
            {
            "fileName": "hctl_linux_i386.tar.gz",
            "algo": "Sha256",
            "source": "hctl_0.3.1_checksums.txt",
            "hash": "4b21ceacabbb4a9009d9b732e3ea4c24266efc09a9e4cc92be05177250064b8c"
            },
            {
            "fileName": "hctl_linux_x86_64.tar.gz",
            "algo": "Sha256",
            "source": "hctl_0.3.1_checksums.txt",
            "hash": "5ab2abdd2f8cbc47f85a14c959d130aabad3ba6bbd75b7092a75cee7871c1158"
            },
            {
            "fileName": "hctl_windows_arm64.zip",
            "algo": "Sha256",
            "source": "hctl_0.3.1_checksums.txt",
            "hash": "deba0f6f5eec9233aaddc06446076bb8a841cedd5c1ef3daa2d53f785b03844c"
            },
            {
            "fileName": "hctl_windows_i386.zip",
            "algo": "Sha256",
            "source": "hctl_0.3.1_checksums.txt",
            "hash": "403b69815e48e2fab3f9575ea0e68409d750b5b066075e85a7440957b24a7170"
            },
            {
            "fileName": "hctl_windows_x86_64.zip",
            "algo": "Sha256",
            "source": "hctl_0.3.1_checksums.txt",
            "hash": "84d902f5516f597057ba4c5806d55093ae0e40520e3c3340adfa7c4ccdab42f9"
            }
        ]
        }
    "#;
}
