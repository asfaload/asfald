use url::Url;

pub fn pause() {
    let mut s = "".to_string();
    println!("Pausing test, press enter when done");
    let _ = std::io::stdin().read_line(&mut s);
}

#[tokio::test]
async fn test_download_and_verify() {
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

    // Set environment variables
    std::env::set_var("GITHUB_API_KEY", "test_token");

    // Create custom GitHub client with mock server URL
    let github_client =
        asfald::GitHubClient::new().with_api_urls(url::Url::parse(server.url().as_str()).unwrap());

    // Create downloader with custom client
    let downloader = asfald::Downloader::new().with_client(github_client);

    let address = format!(
        "{}/test/repo/releases/download/v1.0.0/test-file.tar.gz",
        server.url()
    );

    let url = Url::parse(&address).unwrap();

    // Use the downloader directly instead of CLI
    match downloader.download_and_verify(url, None, false).await {
        Ok(result) => {
            assert_eq!(result.algorithm.to_string(), "sha256");
            assert_eq!(
                result.hash,
                "6ae8a75555209fd6c44157c0aed8016e763ff435a19cf186f76863140143ff72"
            );
        }
        Err(e) => {
            pause();
            panic!("Download failed: {}", e)
        }
    }

    // Clean up
    mock.assert();
    file_mock.assert();
    std::env::remove_var("GITHUB_API_KEY");
}
