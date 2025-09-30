use std::path::PathBuf;
use std::str::FromStr;

use asfald::{DownloadResult, Downloader, HashAlgorithm, Hasher};
use url::Url;

pub fn pause() {
    let mut s = "".to_string();
    println!("Pausing test, press enter when done");
    let _ = std::io::stdin().read_line(&mut s);
}

struct GithubMock {
    cleanup: Box<dyn FnOnce()>,
    downloader: Downloader,
    url: Url,
    expected: DownloadResult,
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
                    "browser_download_url": "{}/test/repo/releases/download/v1.0.0/test-file.tar.gz"
                }}
            ]
        }}
        "#, server.url()))
        .create();

    // Mock file download
    let file_mock = server
        .mock(
            "GET",
            "/test/repo/releases/download/v1.0.0/test-file.tar.gz",
        )
        .with_status(200)
        .with_body(b"test content")
        .create();

    // Create custom GitHub client with mock server URL
    let github_client =
        asfald::GitHubClient::new().with_api_urls(url::Url::parse(server.url().as_str()).unwrap());

    let downloader = asfald::Downloader::new().with_client(github_client);

    let address = format!(
        "{}/test/repo/releases/download/v1.0.0/test-file.tar.gz",
        server.url()
    );
    let cleanup = move || {
        // Moving the server in the cleanup keeps it alive until
        // the lambda is called
        let _s = server;
        mock.assert();
        file_mock.assert();
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
        downloader,
        url,
        expected,
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
