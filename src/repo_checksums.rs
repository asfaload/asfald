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

pub async fn file_checksum_from(url: Url, file: &str) -> Result<String, anyhow::Error> {
    let data = fetch_checksum_url(url).await?;
    let checksum = data.parse::<Checksum>()?;
    let r = checksum.for_file(file)?;
    Ok(r)
}
