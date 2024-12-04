// Code to user asfaload index files
mod json;
use json::v1::{self, FileChecksum};

use crate::{
    asfaload_mirror,
    checksum::{self, ChecksumValidator},
    file_checksum_from,
    logger::helpers::{log_info, log_warn},
    utils,
};

const INDEX_NAME: &str = "asfaload.index.json";

pub fn index_for(url: &url::Url) -> url::Url {
    let mirror_host = asfaload_mirror::choose();
    let on_mirror = asfaload_mirror::url_on_mirror(mirror_host, url);
    on_mirror.join(INDEX_NAME).unwrap()
}

pub fn original_checksums_file_for(
    download_url: &url::Url,
    file_checksum: &FileChecksum,
) -> url::Url {
    download_url.join(file_checksum.source.as_str()).unwrap()
}
// optional indicates if we continue even if we cannot validated the checksum
pub async fn checksum_for(url: &url::Url, optional: bool) -> anyhow::Result<ChecksumValidator> {
    let index_url = index_for(url);
    let filename = url
        .path_segments()
        .unwrap()
        .last()
        .expect("Cannot extract filename from url");
    let response = utils::fetch_url(index_url).await?.error_for_status()?;
    let index: v1::AsfaloadIndex = response.json().await?;
    let hash_info = index
        .best_hash(filename)
        .expect("Didn't find checksum for file in index file");
    let original_checksums_file_url = original_checksums_file_for(url, hash_info);
    let release_checksum =
        file_checksum_from(original_checksums_file_url, hash_info.file_name.as_str())
            .await
            .unwrap();
    if release_checksum != hash_info.hash {
        if optional {
            log_warn("Checksum found in release is different, but continuing as --force-invalid flag found");
        } else {
            anyhow::bail!("Checksum found on mirror is different from checksum found in release. Was release updated?")
        }
    } else {
        log_info("Same checksum found in release");
    }

    // FIXME: we should not duplicate algos types definitions. Needs a refactor of checksums.rs
    let checksum_algorithm = match hash_info.algo {
        v1::Algo::Sha256 => checksum::ChecksumAlgorithm::SHA256,
        v1::Algo::Sha512 => checksum::ChecksumAlgorithm::SHA512,
        v1::Algo::Md5 => checksum::ChecksumAlgorithm::MD5,
        v1::Algo::Sha1 => checksum::ChecksumAlgorithm::SHA1,
    };
    let validator = checksum::ChecksumValidator::new(checksum_algorithm, hash_info.hash.as_str());
    Ok(validator)
}

#[cfg(test)]
mod asfaload_index_tests {

    use anyhow::Result;

    use super::*;

    // We will test with asfald's v0.3.0 index
    const INDEX_JSON: &str = r#"{"mirroredOn":"2024-11-08T15:50:17.5034034+00:00","publishedOn":"2024-11-08T14:01:08+00:00","version":1,"publishedFiles":[{"fileName":"asfald-aarch64-apple-darwin","algo":"Sha256","source":"checksums.txt","hash":"b2ad8f03807b15335dd2af367b55d6318ffe46d32462e514c272272c9aeba130"},{"fileName":"asfald-aarch64-apple-darwin.tar.gz","algo":"Sha256","source":"checksums.txt","hash":"6c1cba9e7da41f9c047bd7ee58f2015fe7efc3b45c3b57c67f19ebf69629d5d1"},{"fileName":"asfald-aarch64-unknown-linux-musl","algo":"Sha256","source":"checksums.txt","hash":"3ae752805a8d4f05091deea144e969b5f5e1ed0e523f9a02a967bc1b01fccc93"},{"fileName":"asfald-aarch64-unknown-linux-musl.tar.gz","algo":"Sha256","source":"checksums.txt","hash":"b17eda226cf8fe3320048fcef40138e64a3d7b9aaa025bb59ba87b2868439ad0"},{"fileName":"asfald-armv7-unknown-linux-musleabi","algo":"Sha256","source":"checksums.txt","hash":"7f6a59637d9461f91a26e022fa5536d9f84780f3c90c1f4816db05140ddc74ec"},{"fileName":"asfald-armv7-unknown-linux-musleabi.tar.gz","algo":"Sha256","source":"checksums.txt","hash":"c6a3c7d3e11a6fe470f2de45e4cffa4d647119901c9d994e91bd29d0e40a24cc"},{"fileName":"asfald-x86_64-apple-darwin","algo":"Sha256","source":"checksums.txt","hash":"7e215f1a9a2934827dbb44ab82567eef68ab020cd4d6642c1a59f5f666f67d56"},{"fileName":"asfald-x86_64-apple-darwin.tar.gz","algo":"Sha256","source":"checksums.txt","hash":"d6f93c508a2c7185b9b6cde87ecc0a6b05fccab10af1ed647c93c7c3bacba3f3"},{"fileName":"asfald-x86_64-pc-windows-msvc.zip","algo":"Sha256","source":"checksums.txt","hash":"a020d7e434ca29c0b5ce10fca8542db33ac373fa4434a4c28975cf07b1d39b98"},{"fileName":"asfald-x86_64-unknown-freebsd","algo":"Sha256","source":"checksums.txt","hash":"badaee9802db53c23a65107bcc1505237f4aaf6d75925829ba3383af90559c95"},{"fileName":"asfald-x86_64-unknown-freebsd.tar.gz","algo":"Sha256","source":"checksums.txt","hash":"793d8032b143092148bd6ae84541a6c6ccd550f828cb81b45f6dc4f07f2d556e"},{"fileName":"asfald-x86_64-unknown-linux-musl","algo":"Sha256","source":"checksums.txt","hash":"88406610b547d5ee107f1e169ac2e455beee131a2b81350a52e5d609a8e5f421"},{"fileName":"asfald-x86_64-unknown-linux-musl.tar.gz","algo":"Sha256","source":"checksums.txt","hash":"582ad5ea1a502071dd469d09a4ae649c985cbc992861f0897084f31826257f87"}]}"#;
    const INDEX_ADDRESS : &str = "https://gh.checksums.asfaload.com/github.com/asfaload/asfald/releases/download/v0.3.0/index.asfaload.json";
    const DOWNLOAD_ADDRESS : &str= "https://github.com/asfaload/asfald/releases/download/v0.3.0/asfald-x86_64-unknown-linux-musl.tar.gz";
    const FILENAME: &str = "asfald-x86_64-unknown-linux-musl.tar.gz";
    const CHECKSUMS_FILE: &str = r#"b2ad8f03807b15335dd2af367b55d6318ffe46d32462e514c272272c9aeba130  asfald-aarch64-apple-darwin
6c1cba9e7da41f9c047bd7ee58f2015fe7efc3b45c3b57c67f19ebf69629d5d1  asfald-aarch64-apple-darwin.tar.gz
3ae752805a8d4f05091deea144e969b5f5e1ed0e523f9a02a967bc1b01fccc93  asfald-aarch64-unknown-linux-musl
b17eda226cf8fe3320048fcef40138e64a3d7b9aaa025bb59ba87b2868439ad0  asfald-aarch64-unknown-linux-musl.tar.gz
7f6a59637d9461f91a26e022fa5536d9f84780f3c90c1f4816db05140ddc74ec  asfald-armv7-unknown-linux-musleabi
c6a3c7d3e11a6fe470f2de45e4cffa4d647119901c9d994e91bd29d0e40a24cc  asfald-armv7-unknown-linux-musleabi.tar.gz
7e215f1a9a2934827dbb44ab82567eef68ab020cd4d6642c1a59f5f666f67d56  asfald-x86_64-apple-darwin
d6f93c508a2c7185b9b6cde87ecc0a6b05fccab10af1ed647c93c7c3bacba3f3  asfald-x86_64-apple-darwin.tar.gz
a020d7e434ca29c0b5ce10fca8542db33ac373fa4434a4c28975cf07b1d39b98  asfald-x86_64-pc-windows-msvc.zip
badaee9802db53c23a65107bcc1505237f4aaf6d75925829ba3383af90559c95  asfald-x86_64-unknown-freebsd
793d8032b143092148bd6ae84541a6c6ccd550f828cb81b45f6dc4f07f2d556e  asfald-x86_64-unknown-freebsd.tar.gz
88406610b547d5ee107f1e169ac2e455beee131a2b81350a52e5d609a8e5f421  asfald-x86_64-unknown-linux-musl
582ad5ea1a502071dd469d09a4ae649c985cbc992861f0897084f31826257f87  asfald-x86_64-unknown-linux-musl.tar.gz"#;

    // Helper to get the AsfaloadIndex used in tests
    fn index() -> v1::AsfaloadIndex {
        serde_json::from_str(INDEX_JSON).unwrap()
    }

    #[test]
    fn test_index_for() -> Result<()> {
        let download_url = url::Url::parse(DOWNLOAD_ADDRESS)?;
        let possible_indexes = ["http://localhost:9898/github.com/asfaload/asfald/releases/download/v0.3.0/asfaload.index.json","http://localhost:9899/github.com/asfaload/asfald/releases/download/v0.3.0/asfaload.index.json"];
        let mirror_url = index_for(&download_url);
        assert!(possible_indexes
            .iter()
            .any(|i| *i == mirror_url.to_string().as_str()));
        Ok(())
    }
    #[test]
    fn test_original_checksums_file_for() -> Result<()> {
        let download_url = url::Url::parse(DOWNLOAD_ADDRESS)?;
        let index = index();
        let file_checksum = index.best_hash(FILENAME).unwrap();
        let original_checksum_url = original_checksums_file_for(&download_url, file_checksum);
        assert_eq!(
            "https://github.com/asfaload/asfald/releases/download/v0.3.0/checksums.txt",
            original_checksum_url.to_string()
        );
        Ok(())
    }
}
