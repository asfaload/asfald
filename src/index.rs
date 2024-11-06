// Code to user asfaload index files
#[allow(dead_code)]
// FIXME: why can't I use the asfaload_mirror module like the utils module?
// Even when declaring the mod asfaload_mirror in lib.rs, I cannot use crate::asfaload_mirror....
// But this causes a warning that asfload_mirror is loaded multiple times
#[path = "asfaload_mirror.rs"]
mod asfaload_mirror;
use crate::checksum;
use crate::checksum::ChecksumValidator;
use crate::utils;
mod json;
use json::v1;

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

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_index_for() -> Result<()> {
        let download_url = url::Url::parse("https://github.com/asfaload/asfald/releases/download/v0.2.0/asfald-x86_64-unknown-linux-musl.tar.gz")?;
        let possible_indexes = ["https://gh.checksums.asfaload.com/github.com/asfaload/asfald/releases/download/v0.2.0/asfaload.index.json","https://gh.checksums.asfaload.com/github.com/asfaload/asfald/releases/download/v0.2.0/asfaload.index.json"];
        let mirror_url = index_for(&download_url);
        assert!(possible_indexes
            .iter()
            .any(|i| *i == mirror_url.to_string().as_str()));
        Ok(())
    }
}
