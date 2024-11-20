use anyhow::Context;
use logger::helpers::log_warn;
use once_cell::sync::Lazy;
use url::Url;

use crate::{asfaload_mirror, fetch_url, logger, Checksum, ChecksumValidator};

pub static CHECKSUMS_FILES: Lazy<Vec<String>> = Lazy::new(|| {
    vec![
        // Define checksum file patterns here. Variables are availabe to define the patterns:
        //   - ${{path}}: The target URL path, excluding the filename.
        //   - ${{file}}: The filename of the target URL.
        //   - ${{fullpath}}: The full path, which is the combination of ${{path}} and ${{file}}.
        "${path}/CHECKSUM.txt".to_string(),
        "${path}/checksum.txt".to_string(),
        "${path}/CHECKSUMS.txt".to_string(),
        "${path}/CHECKSUMS256.txt".to_string(),
        "${path}/checksums.txt".to_string(),
        "${path}/SHASUMS256.txt".to_string(),
        "${path}/SHASUMS256".to_string(),
        "${fullpath}.sha256sum".to_string(),
        // TODO add more patterns
    ]
});

// Return a new URL with the path updated
pub fn update_url_asfaload_host(url: &Url) -> Url {
    let chosen_host = asfaload_mirror::choose();
    let mut nurl = url.clone();
    let npath = asfaload_mirror::path_on_mirror(chosen_host, url);
    nurl.set_path(&npath);
    let host_result = nurl.set_host(Some(chosen_host.host));
    host_result
        .map(|_| -> Url { nurl })
        .context("Error setting asfaload host in checksums url")
        .unwrap()
}
// Return a new URL with the path updated
pub fn update_url_path(url: &Url, path: &str) -> Url {
    let mut nurl = url.clone();
    // The path is always considered from the root.
    // We add the / here so the url is constructed correctly later on.
    let root_path = if path.starts_with('/') {
        path.to_string()
    } else {
        "/".to_string() + path
    };
    nurl.set_path(&root_path);
    nurl
}
// Return the checksums file url that will be used when downloading the file at url
// and using the location 'path' to find the checksums file.
pub fn handle_pattern(url: &url::Url, path: &str) -> std::option::Option<url::Url> {
    // Try to parse the location 'path' pointing to the checksums file to determin
    // if we got a URL or a path on the same server.
    let url_result = Url::parse(path);
    match url_result {
        // If the url could be parsed, check the result's validity
        Ok(u) => {
            // If the scheme is http or https, we're good
            if u.scheme() == "http" || u.scheme() == "https" {
                Some(u)
            }
            // Otherwise we do not have a usable url
            else {
                log_warn("The location of the checksums file given was determined to be a URL, but the scheme is not http/https.");
                None
            }
        }
        // If no url could be parsed, use it as path on the server
        Err(_) => Some(update_url_path(url, path)),
    }
}

// If the path/pattern has the scheme http or https, return it, othewise use it with the url's host
pub fn use_pattern_as_url_if_valid_scheme(url: &url::Url, pattern: &str) -> url::Url {
    // Check if we parse a useable Url. Needed because
    // "httplocalhost:9989/remote/checksums.txt" would be parsed successfully
    // with scheme "httplocalhost"
    let checksums_url = handle_pattern(url, pattern);

    // If we received a pattern starting with http, but the scheme parsed in not http
    // or https, warn the user and use the pattern as path on the server of the file to
    // be downloaded
    match checksums_url {
        Some(u) => u,
        None => {
            log_warn(
                "Checksums file template started with http, \
                      but could not be parsed as URL. Using it as path on same server as file to download.",
            );
            update_url_path(url, pattern)
        }
    }
}

pub async fn fetch_checksum_url(url: url::Url) -> anyhow::Result<String, anyhow::Error> {
    fetch_url(url)
        .await
        .context("Unable to fetch checksum file")?
        .text()
        .await
        .context("Unable to find a checksum file")
}
// Returns a result of tuple (validator,url), so the url can be reported to the user
pub async fn fetch_checksum(url: Url, file: &str) -> anyhow::Result<(ChecksumValidator, Url)> {
    // Clone url as it is moved by fetch_url
    let returned_url = url.clone();
    let data = fetch_checksum_url(url).await?;

    // Parse the file as a checksum:
    match data.parse::<Checksum>() {
        Ok(checksum) => checksum
            .into_validator(file)
            .context(format!("Unable to find '{file}' in checksum"))
            .map(|validator| (validator, returned_url)),
        Err(e) => anyhow::bail!("Failed to parse checksum file: {e:?}"),
    }
}

pub fn file_checksum_from_string(data: String, file: &str) -> Result<String, anyhow::Error> {
    let checksum = data.parse::<Checksum>()?;
    let r = checksum.for_file(file)?;
    Ok(r)
}
pub async fn file_checksum_from(url: Url, file: &str) -> Result<String, anyhow::Error> {
    let data = fetch_checksum_url(url).await?;
    file_checksum_from_string(data, file)
}

#[cfg(test)]
mod asfaload_index_tests {

    use anyhow::Result;

    use super::*;

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

    #[test]
    fn test_file_checksum_from_string() -> Result<()> {
        assert_eq!(
            file_checksum_from_string(
                CHECKSUMS_FILE.to_string(),
                "asfald-aarch64-apple-darwin.tar.gz"
            )
            .unwrap(),
            "6c1cba9e7da41f9c047bd7ee58f2015fe7efc3b45c3b57c67f19ebf69629d5d1"
        );
        assert_eq!(
            file_checksum_from_string(CHECKSUMS_FILE.to_string(), "asfald-x86_64-unknown-freebsd")
                .unwrap(),
            "badaee9802db53c23a65107bcc1505237f4aaf6d75925829ba3383af90559c95"
        );
        assert_eq!(
            file_checksum_from_string(
                CHECKSUMS_FILE.to_string(),
                "asfald-x86_64-unknown-linux-musl.tar.gz"
            )
            .unwrap(),
            "582ad5ea1a502071dd469d09a4ae649c985cbc992861f0897084f31826257f87"
        );
        Ok(())
    }
}
