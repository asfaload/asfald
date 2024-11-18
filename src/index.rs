// Code to user asfaload index files
mod json;
use json::v1;

use crate::{
    asfaload_mirror,
    checksum::{self, ChecksumValidator},
    file_checksum_from,
    logger::helpers::log_info,
    utils,
};

const INDEX_NAME: &str = "asfaload.index.json";

pub fn index_for(url: &url::Url) -> url::Url {
    let mirror_host = asfaload_mirror::choose();
    let on_mirror = asfaload_mirror::url_on_mirror(mirror_host, url);
    on_mirror.join(INDEX_NAME).unwrap()
}

//pub async fn checksum_for(url: url::Url) -> Result<ChecksumValidator, reqwest::Error> {
pub async fn checksum_for(url: &url::Url) -> anyhow::Result<ChecksumValidator> {
    let index_url = index_for(url);
    let filename = url
        .path_segments()
        .unwrap()
        .last()
        .expect("Cannot extract filename from url");
    let response = utils::fetch_url(index_url).await?.error_for_status()?;
    let index: v1::AsfaloadIndex = response.json().await?;
    let hash = index
        .best_hash(filename)
        .expect("Didn't find checksum for file in index file");
    let original_checksums_file_url = url.join(hash.source.as_str()).unwrap();
    let release_checksum = file_checksum_from(original_checksums_file_url, hash.file_name.as_str())
        .await
        .unwrap();
    if release_checksum != hash.hash {
        anyhow::bail!("Checksum found on mirror is different from checksum found in release. Was release updated?")
    } else {
        log_info("Same checksum found in release");
    }

    // FIXME: we should not duplicate algos types definitions. Needs a refactor of checksums.rs
    let checksum_algorithm = match hash.algo {
        v1::Algo::Sha256 => checksum::ChecksumAlgorithm::SHA256,
        v1::Algo::Sha512 => checksum::ChecksumAlgorithm::SHA512,
        v1::Algo::Md5 => checksum::ChecksumAlgorithm::MD5,
        v1::Algo::Sha1 => checksum::ChecksumAlgorithm::SHA1,
    };
    let validator = checksum::ChecksumValidator::new(checksum_algorithm, hash.hash.as_str());
    Ok(validator)
}

#[cfg(test)]
mod asfaload_index_tests {

    use anyhow::Result;

    use super::*;

    #[test]
    fn test_index_for() -> Result<()> {
        let download_url = url::Url::parse("https://github.com/asfaload/asfald/releases/download/v0.2.0/asfald-x86_64-unknown-linux-musl.tar.gz")?;
        let possible_indexes = ["https://gh.checksums.asfaload.com/github.com/asfaload/asfald/releases/download/v0.2.0/asfaload.index.json","https://cf.checksums.asfaload.com/github.com/asfaload/asfald/releases/download/v0.2.0/asfaload.index.json"];
        let mirror_url = index_for(&download_url);
        assert!(possible_indexes
            .iter()
            .any(|i| *i == mirror_url.to_string().as_str()));
        Ok(())
    }
}
