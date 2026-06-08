use crate::{
    Error, Result,
    client::{self, GitHubClient},
    hasher::{HashAlgorithm, Hasher},
};
use client_lib::{ClientLibError, DownloadCallbacks, download_file_with_verification};
use futures::TryStreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Returns true only for errors meaning Asfaload protection is unavailable
/// (network / fetch / forge issues), never for errors where verification
/// actively failed. Any unlisted variant returns false — a safe default that
/// also covers future error variants until they are explicitly classified.
fn allows_github_digest_fallback(err: &ClientLibError) -> bool {
    matches!(
        err,
        ClientLibError::Network(_)
            | ClientLibError::HttpError { .. }
            | ClientLibError::ArtifactInfoFetchError(_)
            | ClientLibError::SignersChainFetchError(_)
            | ClientLibError::SignersChainForgeFetchError(_)
            | ClientLibError::RevocationFetchError(_)
            | ClientLibError::UnsupportedForge(_)
            | ClientLibError::ForgeUrl(_)
    )
}

/// Decide whether to fall back to GitHub-digest verification: only when the
/// user opted in, the URL is a GitHub release URL, and the failure indicates
/// Asfaload protection is unavailable.
fn should_fallback(err: &ClientLibError, github_fallback: bool, is_github_release: bool) -> bool {
    github_fallback && is_github_release && allows_github_digest_fallback(err)
}

pub struct Downloader {
    pub client: GitHubClient,
    pub progress_init: Box<dyn Fn(u64) -> ProgressBar>,
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

impl Downloader {
    pub fn new() -> Self {
        let progress_init = |size| {
            let pb = ProgressBar::new(size);
            pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap_or(indicatif::ProgressStyle::default_bar())
            .progress_chars("#>-"));
            pb
        };
        Self {
            client: GitHubClient::new(),
            progress_init: Box::new(progress_init),
        }
    }

    // Add method to set custom GitHub client
    pub fn with_client(mut self, client: GitHubClient) -> Self {
        self.client = client;
        self
    }

    pub fn with_progress_init(self, f: impl Fn(u64) -> ProgressBar + 'static) -> Self {
        Self {
            progress_init: Box::new(f),
            ..self
        }
    }

    pub async fn get_release(&self, url: url::Url) -> Result<client::GitHubRelease> {
        let (owner, repo, tag, _filename) =
            GitHubClient::parse_github_url(url.to_string().as_str())?;
        // Get release information
        self.client.get_release(&owner, &repo, &tag).await
    }

    async fn get_asset_from_url(&self, url: &url::Url) -> Result<(client::GitHubAsset, String)> {
        let (owner, repo, tag, filename) =
            GitHubClient::parse_github_url(url.to_string().as_str())?;
        let release = self.client.get_release(&owner, &repo, &tag).await?;
        let asset = self.get_asset_in_release_for_filename(release, filename.clone())?;
        Ok((asset, filename))
    }

    pub fn get_asset_in_release_for_filename(
        &self,
        release: client::GitHubRelease,
        filename: String,
    ) -> Result<client::GitHubAsset> {
        release
            .assets
            .into_iter()
            .find(|a| a.name == filename)
            .ok_or_else(|| Error::AssetNotFound(filename.clone()))
    }

    pub async fn get_hash_for_url(&self, url: url::Url) -> Result<String> {
        let (asset, _) = self.get_asset_from_url(&url).await?;
        Ok(asset.digest)
    }
    pub async fn download_and_verify(
        &self,
        url: url::Url,
        output_path: Option<&Path>,
        quiet: bool,
    ) -> Result<DownloadResult> {
        let (asset, filename) = self.get_asset_from_url(&url).await?;

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

    /// Download `url` with full Asfaload verification. Falls back to GitHub
    /// release digest verification only when `github_fallback` is set, `url` is
    /// a GitHub release URL, and the failure means Asfaload protection is
    /// unavailable (never on a verification failure).
    pub async fn download(
        &self,
        url: url::Url,
        output_path: Option<&Path>,
        backend_url: &str,
        forge_type: Option<&str>,
        github_fallback: bool,
        quiet: bool,
    ) -> Result<()> {
        let callbacks = if quiet {
            DownloadCallbacks::default()
        } else {
            DownloadCallbacks::basic()
        };

        let output_owned = output_path.map(Path::to_path_buf);
        let result = download_file_with_verification(
            url.as_str(),
            output_owned.as_ref(),
            backend_url,
            forge_type,
            &callbacks,
        )
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                let is_github_release = GitHubClient::parse_github_url(url.as_str()).is_ok();
                if should_fallback(&e, github_fallback, is_github_release) {
                    if !quiet {
                        eprintln!(
                            "Asfaload verification unavailable; falling back to GitHub release digest."
                        );
                    }
                    // FIXME: We have some repeated download occuring here. We should replace it
                    // with a verify_downloaded_file, but download_file_with_verification might
                    // interrupt before it downloaded the whole file. The good thing is that big
                    // files won't have their download repeated as it will be interrupted as soon
                    // as asfaload auth is detected as absent.
                    self.download_and_verify(url, output_path, quiet)
                        .await
                        .inspect(|r| {
                            if !quiet {
                                println!("Check successful, {} = {}", r.algorithm, r.hash)
                            }
                        })?;
                    Ok(())
                } else {
                    Err(Error::ClientLib(e))
                }
            }
        }
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
            let pb = (self.progress_init)(total_size);
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

#[cfg(test)]
mod fallback_tests {
    use super::*;
    use client_lib::ClientLibError;

    // Representative "protection unavailable" error, used in place of
    // ClientLibError::Network (which needs a live reqwest::Error to construct).
    // Both are eligible for fallback.
    fn unavailable_error() -> ClientLibError {
        ClientLibError::HttpError {
            status: 503,
            url: "https://backend.example".to_string(),
        }
    }

    #[test]
    fn unavailable_errors_are_eligible() {
        assert!(allows_github_digest_fallback(&unavailable_error()));
        assert!(allows_github_digest_fallback(
            &ClientLibError::ArtifactInfoFetchError("x".to_string())
        ));
        assert!(allows_github_digest_fallback(
            &ClientLibError::UnsupportedForge("x".to_string())
        ));
        assert!(allows_github_digest_fallback(
            &ClientLibError::RevocationFetchError("x".to_string())
        ));
        assert!(allows_github_digest_fallback(
            &ClientLibError::SignersChainFetchError("x".to_string())
        ));
        assert!(allows_github_digest_fallback(
            &ClientLibError::SignersChainForgeFetchError("x".to_string())
        ));
    }

    #[test]
    fn verification_failures_are_not_eligible() {
        assert!(!allows_github_digest_fallback(
            &ClientLibError::FileRevoked {
                timestamp: "t".to_string(),
                initiator: "i".to_string(),
            }
        ));
        assert!(!allows_github_digest_fallback(
            &ClientLibError::HashMismatch {
                expected: "e".to_string(),
                computed: "c".to_string(),
            }
        ));
        assert!(!allows_github_digest_fallback(
            &ClientLibError::SignatureThresholdNotMet {
                required: 1,
                found: 0,
            }
        ));
        assert!(!allows_github_digest_fallback(
            &ClientLibError::FileNotInIndex("f".to_string())
        ));
        assert!(!allows_github_digest_fallback(
            &ClientLibError::Unauthorized("u".to_string())
        ));
    }

    #[test]
    fn should_fallback_truth_table() {
        let eligible = unavailable_error();
        let ineligible = ClientLibError::FileRevoked {
            timestamp: "t".to_string(),
            initiator: "i".to_string(),
        };
        // All three conditions must hold.
        assert!(should_fallback(&eligible, true, true));
        // Flag off.
        assert!(!should_fallback(&eligible, false, true));
        // Not a GitHub release URL.
        assert!(!should_fallback(&eligible, true, false));
        // Eligible flag + URL but a verification failure.
        assert!(!should_fallback(&ineligible, true, true));
    }
}
