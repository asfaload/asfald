use std::collections::HashMap;

use anyhow::Context;
use clap::{Args, Parser};
use console::{style, Emoji};
use futures::{future::select_ok, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info, warn, LevelFilter};
use once_cell::sync::Lazy;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use url::Url;

mod checksum;
use checksum::{Checksum, ChecksumValidator};

mod logger;
use logger::Logger;

static CHECKSUMS_FILES: Lazy<Vec<String>> = Lazy::new(|| {
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

static SEARCH: Emoji<'_, '_> = Emoji("🔍", "");
static FOUND: Emoji<'_, '_> = Emoji("✨", "");
static WARN: Emoji<'_, '_> = Emoji("⚠️", "");
static TRASH: Emoji<'_, '_> = Emoji("🗑️", "");
static DOWNLOAD: Emoji<'_, '_> = Emoji("🚚", "");
static VALID: Emoji<'_, '_> = Emoji("✅", "");
static INVALID: Emoji<'_, '_> = Emoji("❌", "");
static ERROR: Emoji<'_, '_> = Emoji("🚨", "/!\\");

static EXAMPLE_HELP: Lazy<String> = Lazy::new(|| {
    format!("
{}

The -p/--pattern <TEMPLATE> flag allows you to specify additional checksum file
patterns to search for, beyond those that the app already looks for by default.
You can repeat this option to search for multiple patterns.

The <TEMPLATE> can either be a full URL path to a checksum file or a template
using predefined variables. These variables are:

 - ${{path}}: The target URL path, excluding the filename.
 - ${{file}}: The filename of the target URL.
 - ${{fullpath}}: The full path, which is the combination of ${{path}} and ${{file}}.

Searching for Checksums ending with .checksum:

 $ asfd -p \"\\${{fullpath}}.checksum\" https://github.com/user/repo/releases/download/v0.0.1/mybinary

This will look for a possible checksum at the following URL:
https://github.com/user/repo/releases/download/v0.0.1/mybinary.checksum

Specifying a full checksum URL:

 $ asfd -p https://another.com/CHECKSUM https://github.com/user/repo/releases/download/v0.0.1/mybinary

", style("Examples:").bold().underlined())
});

fn log_step(emoji: Emoji<'_, '_>, msg: &str) {
    info!("{} {}", emoji, msg);
}

fn log_err(msg: &str) {
    error!("{} {}", ERROR, style(msg).bold().red());
}

fn log_warn(msg: &str) {
    warn!("{} {}", WARN, style(msg).bold().yellow());
}

#[derive(Parser)]
#[command(
    name = "asfd",
    about = "Download a file from a URL and check its checksum",
    after_help = EXAMPLE_HELP.as_str()
)]
struct Cli {
    #[command(flatten)]
    checksum_source: ChecksumSource,
    /// Do not print any output
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Force download even if the checksum is not found
    #[arg(short = 'f', long = "force-absent")]
    force_absent: bool,

    /// Force download even if the checksum is invalid or not found
    #[arg(short = 'F', long = "force-invalid")]
    force_invalid: bool,

    /// Specify the output file
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    output: Option<String>,

    /// The URL to download the file from
    url: Url,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct ChecksumSource {
    /// Specify additional checksums or checksum patterns to search for
    #[arg(short = 'p', long = "pattern", value_name = "TEMPLATE")]
    checksum_patterns: Vec<String>,
}

async fn fetch_url(url: Url) -> Result<reqwest::Response, reqwest::Error> {
    reqwest::get(url).await?.error_for_status()
}

async fn fetch_checksum(url: Url, file: &str) -> anyhow::Result<ChecksumValidator> {
    let data = fetch_url(url)
        .await
        .context("Unable to fetch checksum file")?
        .text()
        .await
        .context("Unable to find a checksum file")?;

    // Parse the file as a checksum:
    match data.parse::<Checksum>() {
        Ok(checksum) => checksum
            .into_validator(file)
            .context(format!("Unable to find '{file}' in checksum")),
        Err(e) => anyhow::bail!("Failed to parse checksum file: {e:?}"),
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        log_err(e.to_string().as_str());
        std::process::exit(1);
    }
}

// Return a new URL with the path updated
fn update_url_path(url: &Url, path: &str) -> Url {
    let mut nurl = url.clone();
    // The path is always considered from the root.
    // We add the / here so the url is constructed correctly later on.
    let root_path = if path.starts_with("/") {
        path.to_string()
    } else {
        "/".to_string() + path
    };
    nurl.set_path(&root_path);
    nurl
}

// Return the checksums file url that will be used when downloading the file at url
// and using the location 'path' to find the checksums file.
fn handle_pattern(url: &url::Url, path: &str) -> std::option::Option<url::Url> {
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
async fn run() -> anyhow::Result<()> {
    let args = Cli::parse();
    let checksum_flag = &args.checksum_source;
    let url = args.url;

    // Initialise the logger:
    let log_level = if args.quiet {
        LevelFilter::Off
    } else {
        LevelFilter::Info
    };
    Logger::new().with_level(log_level).init().unwrap();

    let url_path = url
        .path_segments()
        .map(|c| c.map(|s| s.to_owned()).collect::<Vec<_>>())
        .unwrap_or_else(std::vec::Vec::new);

    let file = url_path.last().context("No file found in URL")?.to_owned();
    let path = url_path[..url_path.len() - 1].join("/");

    log_step(SEARCH, "Looking for checksum files...");

    // Create a hashmap with the path and file to be used in the templates
    let vars = HashMap::from([
        ("fullpath".to_string(), url_path.join("/")),
        ("path".to_string(), path),
        ("file".to_string(), file.clone()),
    ]);
    // This shouldn't happen:
    envsubst::validate_vars(&vars).context("unable to validate substitution variables")?;

    // Create a stream of checksum downloads
    let checksums_patterns = CHECKSUMS_FILES
        .iter()
        .chain(checksum_flag.checksum_patterns.iter())
        // It is safe to unwrap as the only possible error is catched by the validate_vars above
        .map(|tmpl| envsubst::substitute(tmpl, &vars).unwrap())
        // Build the URL where to get the checksums file.
        .map(|path| {
            // Helper to build the replace the path of url by the path passed as argument
            // Template is supposedly a full url
            if path.starts_with("http") {
                // Check if we parse a useable Url. Needed because
                // "httplocalhost:9989/remote/checksums.txt" would be parsed successfully
                // with scheme "httplocalhost"
                let checksums_url = handle_pattern(&url,&path);

                // If we received a pattern starting with http, but the scheme parsed in not http
                // or https, warn the user and use the pattern as path on the server of the file to
                // be downloaded
                match checksums_url {
                    Some(u) => u,
                    None => {
                        log_warn(
                            "Checksums file template started with http, but could not be parsed as URL. Using it as path on same server as file to download.",
                        );
                        update_url_path(&url,&path)
                    }
                }
            }
            // Template is a path, look on same server as file
            else {
                update_url_path(&url,&path)
            }
        })
        .map(|url| Box::pin(fetch_checksum(url, &file)));

    // Select the first checksum file that is found
    let mut checksum = match select_ok(checksums_patterns).await {
        Ok((checksum, _)) => {
            log_step(FOUND, "Checksum file found !");
            Some(checksum)
        }
        Err(e) => {
            if args.force_absent || args.force_invalid {
                log_warn("Checksum file not found, but continuing due to --force-absent or --force-invalid flag");
                None
            } else {
                return Err(e);
            }
        }
    };

    log_step(TRASH, "Create temporary file...");

    // Create a temporary file to store the downloaded data
    let temp_dir = tempfile::tempdir().context("Unable to create a temporary directory")?;
    let temp_file_path = temp_dir.path().join(&file);
    let mut temp_file = File::create(&temp_file_path)
        .await
        .context("Unable to create a temporary file")?;

    log_step(DOWNLOAD, "Downloading file...");

    // Download the file form url and while downloading compute the checksum:
    let response = fetch_url(url).await.context("Download failed")?;

    let file_size = response.content_length();
    let mut data = response.bytes_stream();

    let pb = if args.quiet {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(file_size.unwrap_or(0))
    };

    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta_precise})")
            .unwrap()
            .progress_chars(file_size.map_or(".>.", |_| "#>.")));

    // Download the file and compute the checksum
    while let Some(chunk) = data.next().await {
        match chunk {
            Ok(chunk) => {
                if let Some(ref mut checksum) = checksum {
                    checksum.update(&chunk);
                }
                temp_file
                    .write(&chunk)
                    .await
                    .context("Unable to write to temporary file")?;

                // Update displayed size of the downloaded file, if the content length was unknown
                if file_size.is_none() {
                    pb.set_length(pb.position());
                }
                pb.inc(chunk.len() as u64);
            }
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    pb.finish_with_message("downloaded");

    if let Some(checksum) = checksum.take() {
        // Validate the checksum, if it fails and the force flag is not set, return early
        if checksum.validate().is_err() {
            log_step(INVALID, "File's checksum is invalid !");
            if args.force_invalid {
                log_warn("... but continuing due to --force-invalid flag");
                log_warn("⚠️⚠️ WARNING: this is insecure, and still downloads file with a checksum present, but invalid! ⚠️⚠️");
            } else {
                anyhow::bail!("Checksum validation failed");
            }
        } else {
            log_step(VALID, "File's checksum is valid !");
        }
    }

    // Move the temporary file to the destination file
    let dest_file = args.output.unwrap_or(file);
    fs::rename(temp_file_path, &dest_file)
        .await
        .context(format!("Unable to move the temporary file to {dest_file}"))?;

    Ok(())
}

#[cfg(test)]
mod helpers_tests {
    use super::*;

    #[test]
    fn test_update_url_path() {
        let new_path = "/asfd-checksums/v0.0.1";
        let url1 = Url::parse("http://www.asfaload.com/blog").unwrap();
        let url2 = update_url_path(&url1, new_path);
        assert_eq!(url1.path(), "/blog");
        assert_eq!(url2.path(), new_path);
    }
    #[test]
    fn test_handle_pattern_https_same_server() {
        let checksum_input = "https://www.example.com/public/SHA256SUMS";
        let file_url = Url::parse("https://www.example.com/public/my_file.txt").unwrap();
        let o = handle_pattern(&file_url, checksum_input);
        assert!(o.is_some());
        assert_eq!(checksum_input, o.as_ref().unwrap().to_string());
        assert_eq!("https", o.as_ref().unwrap().scheme());
    }
    #[test]
    fn test_handle_pattern_https_other_server() {
        let checksum_input = "https://www.internal.example.com/checksums/public/SHA256SUMS";
        let file_url = Url::parse("https://www.example.com/public/my_file.txt").unwrap();
        let o = handle_pattern(&file_url, checksum_input);
        assert!(o.is_some());
        assert_eq!(checksum_input, o.as_ref().unwrap().to_string());
        assert_eq!("https", o.as_ref().unwrap().scheme());
    }
    #[test]
    fn test_handle_pattern_path() {
        let checksum_input = "/checksums/public/extended/SHA256SUMS";
        let file_url = Url::parse("https://www.example.com/public/my_file.txt").unwrap();
        let o = handle_pattern(&file_url, checksum_input);
        assert!(o.is_some());
        assert_eq!(
            "https://www.example.com/checksums/public/extended/SHA256SUMS",
            o.as_ref().unwrap().to_string()
        );
        assert_eq!("https", o.as_ref().unwrap().scheme());
    }
}
