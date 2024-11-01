#[allow(dead_code)]
mod v1 {
    use std::{collections::HashMap};

    use ::serde::{Deserialize, Serialize};
    use chrono::DateTime;
    use itertools::Itertools;

    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize, Clone)]
    pub enum Algo {
        Md5,
        Sha1,
        Sha256,
        Sha512,
    }

    impl Algo {
        pub fn iter<'a>() -> std::slice::Iter<'a, Algo> {
            const VALUES: [Algo; 4] = [Algo::Sha512, Algo::Sha256, Algo::Sha1, Algo::Md5];
            VALUES.iter()
        }
    }

    #[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct FileChecksum {
        pub file_name: String,
        pub algo: Algo,
        pub source: String,
        pub hash: String,
    }
    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct AsfaloadIndex {
        pub mirrored_on: DateTime<chrono::Utc>,
        pub published_on: DateTime<chrono::Utc>,
        pub version: i32,
        published_files: Vec<FileChecksum>,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum ChecksumError {
        NotFound,
        MultipleValues,
    }

    pub enum ChecksumsForFile<'a> {
        Consistent(Vec<&'a FileChecksum>),
        Inconsistent(Vec<&'a FileChecksum>),
    }

    impl<'a> ChecksumsForFile<'a> {
        pub fn into_inner(self) -> Vec<&'a FileChecksum> {
            match self {
                ChecksumsForFile::Consistent(vec) => vec,
                ChecksumsForFile::Inconsistent(vec) => vec,
            }
        }

        pub fn as_mut(&mut self) -> &mut Vec<&'a FileChecksum> {
            match self {
                ChecksumsForFile::Consistent(vec) => vec,
                ChecksumsForFile::Inconsistent(vec) => vec,
            }
        }
    }

    pub trait FileChecksumIterator<T>: Sized + Iterator<Item = T> {
        fn file(self, file_name: &str) -> impl FileChecksumIterator<T>;
        fn algo(self, algo: Algo) -> impl FileChecksumIterator<T>;
        fn unique_hash(self) -> impl FileChecksumIterator<T>;
        fn sort_by_algo(self) -> impl FileChecksumIterator<T>;
    }

    impl<'a, I: Iterator<Item = &'a FileChecksum>> FileChecksumIterator<&'a FileChecksum> for I {
        fn file(self, file_name: &str) -> impl FileChecksumIterator<&'a FileChecksum> {
            self.filter(move |item| item.file_name == file_name)
        }

        fn algo(self, algo: Algo) -> impl FileChecksumIterator<&'a FileChecksum> {
            self.filter(move |item| item.algo == algo)
        }

        fn unique_hash(self) -> impl FileChecksumIterator<&'a FileChecksum> {
            self.unique_by(|c| c.hash.as_str())
        }

        fn sort_by_algo(self) -> impl FileChecksumIterator<&'a FileChecksum> {
            self.sorted_by(|a, b| b.algo.cmp(&a.algo))
        }
    }

    impl<I: Iterator<Item = FileChecksum>> FileChecksumIterator<FileChecksum> for I {
        fn file(self, file_name: &str) -> impl FileChecksumIterator<FileChecksum> {
            self.filter(move |item| item.file_name == file_name)
        }

        fn algo(self, algo: Algo) -> impl FileChecksumIterator<FileChecksum> {
            self.filter(move |item| item.algo == algo)
        }

        fn unique_hash(self) -> impl FileChecksumIterator<FileChecksum> {
            self.unique_by(|c| c.hash.to_string())
        }

        fn sort_by_algo(self) -> impl FileChecksumIterator<FileChecksum> {
            self.sorted_by(|a, b| a.algo.cmp(&b.algo))
        }
    }

    impl AsfaloadIndex {
        pub fn iter(&self) -> impl FileChecksumIterator<&FileChecksum> {
            self.published_files.iter()
        }

        pub fn into_iter(self) -> impl FileChecksumIterator<FileChecksum> {
            self.published_files.into_iter()
        }

        pub fn hash(&self, filename: &str, algo: Algo) -> Result<&FileChecksum, ChecksumError> {
            let mut iter = self.iter().file(filename).algo(algo).unique_hash();
            match iter.next() {
                Some(first) => {
                    if iter.next().is_none() {
                        Ok(first)
                    } else {
                        Err(ChecksumError::MultipleValues)
                    }
                }
                None => Err(ChecksumError::NotFound),
            }
        }

        // Return one hash found for the file, in the order of preference Sha512, Sha265, Sha1, Md5
        pub fn best_hash(&self, filename: &str) -> Option<&FileChecksum> {
            self.iter()
                .file(filename)
                .unique_hash()
                .sort_by_algo()
                .next()
        }

        // Return all hashes found for the file, in the order of preference Sha512, Sha265, Sha1, Md5
        // WARNING: does not check consistency, though it signals it in the enum case returned.
        // For example could return 2 different Sha256 hashes found in different checksums files
        pub fn all_hashes(&self, filename: &str) -> ChecksumsForFile {
            self.iter()
                .file(filename)
                .sort_by_algo()
                .fold(
                    (HashMap::new(), ChecksumsForFile::Consistent(Vec::new())),
                    |(mut visited, mut list), item| {
                        if let Some(hash) = visited.get(&item.algo) {
                            if hash != &item.hash {
                                list = ChecksumsForFile::Inconsistent(list.into_inner())
                            }
                        } else {
                            visited.insert(&item.algo, item.hash.clone());
                        }

                        list.as_mut().push(item);

                        (visited, list)
                    },
                )
                .1
        }
    }
}

#[cfg(test)]
mod lib_tests {
    use v1::{AsfaloadIndex, ChecksumError, ChecksumsForFile};

    use super::*;
    use anyhow::Result;
    use chrono::DateTime;

    #[test]
    fn parse_json() -> Result<()> {
        let index: AsfaloadIndex = serde_json::from_str(JSON)?;
        assert_eq!(index.version, 1);
        assert_eq!(
            index.mirrored_on,
            serde_json::from_str::<DateTime<chrono::Utc>>("\"2024-10-30T10:48:24.9397405+00:00\"")?
        );
        Ok(())
    }

    #[test]
    // Retrieve hash of specified type
    fn test_hash() -> Result<()> {
        let index: AsfaloadIndex = serde_json::from_str(JSON)?;

        // Normal situation: one hash is found
        let file_entry = index.hash("hctl_freebsd_arm64.tar.gz", v1::Algo::Sha256);
        assert_eq!(
            file_entry.map(|f| f.hash.as_str()),
            Ok("03ecde4a2efdbfa234b6aaa3ab166ee92e83ffd0d3521b455b51d00ff171909b")
        );

        // Two entries with the same hash values are found, should work fine
        let file_entry = index.hash("hctl_darwin_arm64.tar.gz", v1::Algo::Sha256);
        assert_eq!(
            file_entry.map(|f| f.hash.as_str()),
            Ok("e9e40eeb6c6c049c863cdf8769a8a9553c3739bac5ab1e05444509d676185e6e")
        );

        // Two entries with the same hash values are found, should work fine
        let file_entry = index.hash("hctl_darwin_x86_64.tar.gz", v1::Algo::Sha256);
        assert_eq!(file_entry, Err(ChecksumError::MultipleValues));

        // File has no hash
        let file_entry = index.hash("inexisting.tar.gz", v1::Algo::Sha256);
        assert_eq!(file_entry, Err(ChecksumError::NotFound));

        // Has both Sha512 and Sha256
        let file_entry = index.hash("hctl_freebsd_i386.tar.gz", v1::Algo::Sha256);
        assert_eq!(
            file_entry.map(|f| f.hash.as_str()),
            Ok("d16af5a91631f0c2232c747fa773a8dab21aa896894bbba55847e74a100eec9f")
        );

        let file_entry = index.hash("hctl_freebsd_i386.tar.gz", v1::Algo::Sha512);
        assert_eq!(
            file_entry.map(|f| f.hash.as_str()),
            Ok("d16af5a91631f0c2232c747fa773a8dab21aa896894bbba55847e74a100eec9fd16af5a91631f0c2232c747fa773a8dab21aa896894bbba55847e74a100eec9f")
        );

        Ok(())
    }

    #[test]
    fn test_best_hash() -> Result<()> {
        let index: AsfaloadIndex = serde_json::from_str(JSON)?;

        // Normal situation: one hash is found
        let file_entry = index.best_hash("hctl_freebsd_arm64.tar.gz");
        assert_eq!(
            file_entry.map(|f| f.hash.as_str()),
            Some("03ecde4a2efdbfa234b6aaa3ab166ee92e83ffd0d3521b455b51d00ff171909b")
        );

        // Has both Sha256 and Sha512, should prefer Sha512
        let file_entry = index.best_hash("hctl_freebsd_i386.tar.gz");
        assert_eq!(
            file_entry.map(|f| f.hash.as_str()),
            Some("d16af5a91631f0c2232c747fa773a8dab21aa896894bbba55847e74a100eec9fd16af5a91631f0c2232c747fa773a8dab21aa896894bbba55847e74a100eec9f")
        );

        // File has no hash
        let file_entry = index.best_hash("inexisting.tar.gz");
        assert_eq!(file_entry.map(|f| f.hash.as_str()), None);

        Ok(())
    }

    #[test]
    fn test_all_hashes() -> Result<()> {
        let index: AsfaloadIndex = serde_json::from_str(JSON)?;

        // Two entries with the same hash values are found, return both
        let file_entry = index.all_hashes("hctl_darwin_arm64.tar.gz");

        assert!(matches!(file_entry, ChecksumsForFile::Consistent { .. }));
        if let ChecksumsForFile::Consistent(v) = file_entry {
            assert_eq!(
                v.iter().map(|f| f.hash.clone()).collect::<Vec<String>>(),
                vec![
                    "e9e40eeb6c6c049c863cdf8769a8a9553c3739bac5ab1e05444509d676185e6e".to_string(),
                    "e9e40eeb6c6c049c863cdf8769a8a9553c3739bac5ab1e05444509d676185e6e".to_string()
                ]
            );
            assert_eq!(
                v.iter().map(|f| f.source.clone()).collect::<Vec<String>>(),
                vec![
                    "hctl_0.3.1_checksums.txt",
                    "hctl_0.3.1_checksums.duplicate.txt"
                ]
            );
        }

        // Two entries with differnt hash values for the same file are found, return both
        let file_entry = index.all_hashes("hctl_darwin_x86_64.tar.gz");

        assert!(matches!(file_entry, ChecksumsForFile::Inconsistent { .. }));
        if let ChecksumsForFile::Inconsistent(v) = file_entry {
            assert_eq!(
                v.iter().map(|f| f.hash.clone()).collect::<Vec<String>>(),
                vec![
                    "2bb9254023af4307db99e1f0165e481e54f78e4cf23fa1f169a229ffcc539789".to_string(),
                    "0000000023af4307db99e1f0165e481e54f78e4cf23fa1f169a229ffcc539789".to_string()
                ]
            );
            assert_eq!(
                v.iter().map(|f| f.source.clone()).collect::<Vec<String>>(),
                vec![
                    "hctl_0.3.1_checksums.txt",
                    "hctl_0.3.1_checksums.invalid_duplicate.txt"
                ]
            );
        }

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
            "fileName": "hctl_freebsd_i386.tar.gz",
            "algo": "Sha512",
            "source": "hctl_0.3.1_checksums.txt",
            "hash": "d16af5a91631f0c2232c747fa773a8dab21aa896894bbba55847e74a100eec9fd16af5a91631f0c2232c747fa773a8dab21aa896894bbba55847e74a100eec9f"
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
