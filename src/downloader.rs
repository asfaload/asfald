use crate::{
    client::GitHubClient,
    hasher::{HashAlgorithm, Hasher},
    Error, Result,
};
use futures::TryStreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct Downloader {
    pub client: GitHubClient,
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            client: GitHubClient::new(),
        }
    }

    // Add method to set custom GitHub client
    pub fn with_client(mut self, client: GitHubClient) -> Self {
        self.client = client;
        self
    }
    pub async fn download_and_verify(
        &self,
        url: url::Url,
        output_path: Option<&Path>,
        quiet: bool,
    ) -> Result<DownloadResult> {
        // Parse GitHub URL
        let (owner, repo, tag, filename) =
            GitHubClient::parse_github_url(url.to_string().as_str())?;

        // Get release information
        let release = self.client.get_release(&owner, &repo, &tag).await?;

        // Find the asset
        let asset = release
            .assets
            .into_iter()
            .find(|a| a.name == filename)
            .ok_or_else(|| Error::AssetNotFound(filename.clone()))?;

        // Parse the digest
        let (algorithm, expected_hash) = Hasher::parse_digest(&asset.digest)?;

        // Download the file
        let download_path = output_path.unwrap_or_else(|| Path::new(&filename));
        let actual_hash = self
            .download_file(
                &asset.browser_download_url,
                download_path,
                &algorithm,
                quiet,
            )
            .await?;

        // Verify hash
        if expected_hash != actual_hash {
            return Err(Error::HashVerificationFailed {
                expected: expected_hash,
                actual: actual_hash,
            });
        }

        Ok(DownloadResult {
            path: download_path.to_path_buf(),
            size: std::fs::metadata(download_path)?.len(),
            algorithm,
            hash: actual_hash,
        })
    }

    async fn download_file(
        &self,
        url: &str,
        path: &Path,
        algorithm: &HashAlgorithm,
        quiet: bool,
    ) -> Result<String> {
        let response = reqwest::get(url).await?;

        if !response.status().is_success() {
            return Err(Error::RequestError(
                response.error_for_status().unwrap_err(),
            ));
        }

        let total_size = response
            .content_length()
            .ok_or_else(|| Error::from(std::io::Error::other("Missing content length header")))?;

        let pb = if !quiet {
            let pb = ProgressBar::new(total_size);
            pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap_or(indicatif::ProgressStyle::default_bar())
            .progress_chars("#>-"));
            Some(pb)
        } else {
            None
        };

        let mut file = File::create(path)?;
        let mut stream = response.bytes_stream();

        let mut hasher = match algorithm {
            HashAlgorithm::Sha256 => Sha256::new(),
        };

        while let Some(chunk) = stream.try_next().await? {
            if let Some(pbv) = &pb {
                pbv.inc(chunk.len() as u64);
            }
            hasher.update(&chunk);
            file.write_all(&chunk)?;
        }

        if let Some(pbv) = &pb {
            pbv.finish_with_message("Download complete");
        }

        let hash_result = hasher.finalize();
        Ok(hex::encode(hash_result))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DownloadResult {
    pub path: std::path::PathBuf,
    pub size: u64,
    pub algorithm: HashAlgorithm,
    pub hash: String,
}
