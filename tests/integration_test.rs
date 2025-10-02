use std::str::FromStr;
use std::{path::PathBuf, rc::Rc};

use asfald::{DownloadResult, Downloader, HashAlgorithm, Hasher};
use indicatif::{InMemoryTerm, ProgressBar, ProgressDrawTarget};
use url::Url;

pub fn pause() {
    let mut s = "".to_string();
    println!("Pausing test, press enter when done");
    let _ = std::io::stdin().read_line(&mut s);
}

struct GithubMock {
    cleanup: Box<dyn FnOnce()>,
    server_url: String,
    downloader: Downloader,
    url: Url,
    expected: DownloadResult,
    pb_term: Rc<InMemoryTerm>,
}
impl Drop for GithubMock {
    fn drop(&mut self) {
        // The Drop trait’s drop method takes &mut self, but since FnOnce consumes the closure, you
        // need to take ownership of self.cleanup. To do this, you can use std::mem::replace or
        // std::mem::take to replace the closure with a no-op closure
        let cleanup = std::mem::replace(&mut self.cleanup, Box::new(|| {}));
        cleanup();
    }
}

const TEST_FILE_PATH: &str = "test/repo/releases/download/v1.0.0/test-file.tar.gz";
const TEST_FILE_CONTENT: &[u8] = b"test content";
const INVALID_FILE_PATH: &str = "test/repo/releases/download/v1.0.0/damaged-file.tar.gz";
const INVALID_FILE_CONTENT: &[u8] = b"damaged content";
async fn setup_mocks() -> GithubMock {
    // Mock GitHub API response
    let mut server = mockito::Server::new_async().await;
    let mock = server.mock("GET", "/repos/test/repo/releases/tags/v1.0.0")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(format!(r#"
        {{
            "assets": [
                {{
                    "name": "test-file.tar.gz",
                    "digest": "sha256:6ae8a75555209fd6c44157c0aed8016e763ff435a19cf186f76863140143ff72",
                    "browser_download_url": "{}/{}"
                }},
                {{
                    "name": "damaged-file.tar.gz",
                    "digest": "sha256:00000000000000000000000000000000063ff435a19cf186f76863140143ff72",
                    "browser_download_url": "{}/{}"
                }}
            ]
        }}
        "#, server.url(), TEST_FILE_PATH, server.url(), INVALID_FILE_PATH))
        .create();

    // Mock file download
    let file_mock = server
        .mock("GET", format!("/{}", TEST_FILE_PATH).as_str())
        .with_status(200)
        .with_body(TEST_FILE_CONTENT)
        .expect_at_least(0) // don't report an error if this url was not requested during the test
        .create();

    let invalid_mock = server
        .mock("GET", format!("/{}", INVALID_FILE_PATH).as_str())
        .with_status(200)
        .with_body(INVALID_FILE_CONTENT)
        .expect_at_least(0) // don't report an error if this url was not requested during the test
        .create();
    // Create custom GitHub client with mock server URL
    let github_client =
        asfald::GitHubClient::new().with_api_urls(url::Url::parse(server.url().as_str()).unwrap());

    let in_mem = Rc::new(InMemoryTerm::new(10, 80));
    let term = in_mem.clone();
    let pb_init = move |size| {
        ProgressBar::with_draw_target(Some(size), ProgressDrawTarget::term_like(Box::new(*term)))
    };

    let downloader = asfald::Downloader::new()
        .with_client(github_client)
        .with_progress_init(pb_init);

    let address = format!("{}/{}", server.url(), TEST_FILE_PATH);
    let server_url = server.url();
    let cleanup = move || {
        // Moving the server in the cleanup keeps it alive until
        // the lambda is called
        let _s = server;
        mock.assert();
        file_mock.assert();
        invalid_mock.assert();
    };
    let url = Url::parse(&address).unwrap();

    let expected = DownloadResult {
        algorithm: HashAlgorithm::Sha256,
        hash: Hasher::compute_hash(TEST_FILE_CONTENT, &HashAlgorithm::Sha256).unwrap(),
        path: PathBuf::from_str("test-file.tar.gz").unwrap(),
        size: TEST_FILE_CONTENT.len() as u64,
    };
    GithubMock {
        cleanup: Box::new(cleanup),
        server_url,
        downloader,
        url,
        expected,
        pb_term: in_mem.clone(),
    }
}

#[tokio::test]
async fn test_download_and_verify() {
    let mock_info = setup_mocks().await;
    // Create downloader with custom client

    // Use the downloader directly instead of CLI
    match mock_info
        .downloader
        .download_and_verify(mock_info.url.clone(), None, false)
        .await
    {
        Ok(result) => {
            assert_eq!(result, mock_info.expected);
        }
        Err(e) => {
            panic!("Download failed: {}", e)
        }
    }
}

#[tokio::test]
async fn test_damaged_download_and_verify() {
    let mock_info = setup_mocks().await;

    let downloaded_url = format!("{}/{}", mock_info.server_url, INVALID_FILE_PATH);
    // Use the downloader directly instead of CLI
    match mock_info
        .downloader
        .download_and_verify(Url::parse(downloaded_url.as_str()).unwrap(), None, false)
        .await
    {
        Ok(_) => {
            panic!("Expected an error due to invalid hash")
        }
        Err(e) => match e {
            asfald::Error::HashVerificationFailed {
                expected: _,
                actual,
            } => {
                // Keeping the hard-coded string here, to detect any change that might occur
                let expected_invalid_hash =
                    "4550858567d11b2a79c4c9f585ea220a1389f8265625bb5f6d5a68abcaae6e78";
                assert_eq!(
                    actual, expected_invalid_hash,
                    "Computed hash for damaged file is not the expected value"
                );
            }
            e => panic!("unexpected error type: {}", e),
        },
    }
}
