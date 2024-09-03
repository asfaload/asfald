use std::collections::HashMap;

use anyhow::Context;
use clap::Parser;
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
        "${path}/CHECKSUM.txt".to_string(),
        "${path}/checksum.txt".to_string(),
        "${path}/CHECKSUMS.txt".to_string(),
        "${path}/CHECKSUMS256.txt".to_string(),
        "${path}/checksums.txt".to_string(),
        "${path}/SHASUMS256.txt".to_string(),
        "${path}/SHASUMS256".to_string(),
        "${path}/${file}.sha256sum".to_string(),
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
    name = "downloader",
    about = "Download a file from a URL and check its checksum"
)]
struct Cli {
    /// Do not print any output
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Force download even if the checksum is invalid or not found
    #[arg(short = 'f', long = "force")]
    force: bool,

    /// Specify the output file
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    output: Option<String>,

    /// Specify additional changelogs or patterns to search for
    #[arg(short = 'p', long = "pattern", value_name = "TEMPLATE")]
    changelog_patterns: Vec<String>,

    /// The URL to download the file from
    url: Url,
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

async fn run() -> anyhow::Result<()> {
    let args = Cli::parse();
    let url = args.url;

    // Initialise the logger:
    let log_level = if args.quiet {
        LevelFilter::Off
    } else {
        LevelFilter::Info
    };
    Logger::new()
        .with_level(log_level)
        .error_to_stderr()
        .init()
        .unwrap();

    let url_path = url
        .path_segments()
        .map(|c| c.map(|s| s.to_owned()).collect::<Vec<_>>())
        .unwrap_or_else(std::vec::Vec::new);

    let file = url_path.last().context("No file found in URL")?.to_owned();
    let path = url_path[..url_path.len() - 1].join("/");

    log_step(SEARCH, "Looking for checksum files...");

    // Create a hashmap with the path and file to be used in the templates
    let vars = HashMap::from([
        ("path".to_string(), path),
        ("file".to_string(), file.clone()),
    ]);
    // This shouldn't happen:
    envsubst::validate_vars(&vars).context("unable to validate substitution variables")?;

    // Create a stream of checksum downloads
    let changelog_patterns = CHECKSUMS_FILES
        .iter()
        .chain(args.changelog_patterns.iter())
        // It is safe to unwrap as the only possible error is catched by the validate_vars above
        .map(|tmpl| envsubst::substitute(tmpl, &vars).unwrap())
        .map(|path| {
            // Build the URL for the checksum file & create a future to fetch it
            let mut nurl = url.clone();
            nurl.set_path(&path);
            Box::pin(fetch_checksum(nurl, &file))
        });

    // Select the first checksum file that is found
    let mut checksum = match select_ok(changelog_patterns).await {
        Ok((checksum, _)) => {
            log_step(FOUND, "Checksum file found !");
            Some(checksum)
        }
        Err(e) => {
            if args.force {
                log_warn("Checksum file not found, but continuing due to --force flag");
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
            if args.force {
                log_warn("... but continuing due to --force flag");
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
