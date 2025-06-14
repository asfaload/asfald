use std::{collections::HashMap, error::Error, io::Write, path::Path};

use anyhow::Context;
use asfald::{
    fetch_checksum, fetch_url, index,
    logger::{
        helpers::{
            log_err, log_info, log_step, log_warn, DOWNLOAD, FOUND, INVALID, SEARCH, TRASH, VALID,
        },
        Logger,
    },
    repo_checksums, update_url_asfaload_host, update_url_path, use_pattern_as_url_if_valid_scheme,
    Checksum,
};
use clap::{Args, Parser};
use console::style;
use futures::{future::select_ok, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use log::LevelFilter;
use once_cell::sync::Lazy;

use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
};
use url::Url;
static EXAMPLE_HELP: Lazy<String> = Lazy::new(|| {
    format!("
{}

{}
By default, asfald will look at checksums mirrored at https://github.com/asfaload/checksums and
use an asfaload indexing file.

The previous behaviour, looking for checksums files under well known names, is still available, but
is much less efficient and is strongly discouraged.

The -o <PATH> flag allows to save the file in a particular location. If <PATH> is the single character -,
the downloaded file is sent to standard output after validation.

The -p/--pattern <TEMPLATE> flag allows you to specify additional checksum file
patterns to search for, beyond those that the app already looks for by default.
You can repeat this option to search for multiple patterns.

The <TEMPLATE> can either be a full URL path to a checksum file or a template
using predefined variables. These variables are:

 - ${{path}}: The target URL path, excluding the filename.
 - ${{file}}: The filename of the target URL.
 - ${{fullpath}}: The full path, which is the combination of ${{path}} and ${{file}}.

Searching for Checksums ending with .checksum:

 $ asfald -p \"\\${{fullpath}}.checksum\" https://github.com/user/repo/releases/download/v0.0.1/mybinary

This will look for a possible checksum at the following URL:
https://github.com/user/repo/releases/download/v0.0.1/mybinary.checksum

Specifying a full checksum URL:

 $ asfald -p https://another.com/CHECKSUM https://github.com/user/repo/releases/download/v0.0.1/mybinary

{}
The -H/--hash <HASH> flags allows to pass the hash value to use when validating the downloaded file.
Doing this will allow you to detect when a file you download regularly was modified on the server. This
is especially useful in Dockerfiles.
When this flag is passed, no checksums file will be used.

 $ asfald --hash 87b5fbf82d9258782ffbd141ffbeab954af3ce6ff7a1ad336c70196f40ac233c \\
        https://github.com/asfaload/asfald/releases/download/v0.1.0/asfald-x86_64-unknown-linux-musl
", style("Examples:").bold().underlined(), style("Custom checksums file").underlined(), style("Literal hash value").underlined())
});

#[derive(Parser)]
#[command(
    version,
    name = "asfald",
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

    /// Overwrite existing files
    #[arg(short = 'w', long = "overwrite", value_name = "OVERWRITE")]
    overwrite: bool,

    /// The URL to download the file from
    url: Url,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct ChecksumSource {
    /// Specify additional checksums or checksum patterns to search for
    #[arg(short = 'p', long = "pattern", value_name = "TEMPLATE")]
    checksum_patterns: Vec<String>,
    /// Specify the checksum value for the downloaded file
    #[arg(short = 'H', long = "hash", value_name = "HASH")]
    hash_value: Option<String>,
    /// Specify the host checksums files will be fetched from. Prevents use of asfaload index
    /// files.
    #[arg(short = 'a', long = "asfaload-host", value_name = "WITH_ASFALOAD_HOST")]
    asfaload_host: bool,
    // Do not use asfaload index files.
    #[arg(
        short = 'I',
        long = "no-asfaload-index",
        value_name = "WITHOUT_ASFALOAD_INDEX"
    )]
    no_asfaload_index: bool,
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        log_err(e.to_string().as_str());
        std::process::exit(1);
    }
}

// Helper to log_info details about vulnerability window
fn print_vulnerability_window_info(vuln_window: &chrono::TimeDelta) {
    match (
        vuln_window.num_days(),
        vuln_window.num_hours(),
        vuln_window.num_minutes(),
    ) {
        (0, 0, m) => log_info(
            format!(
                "Vulnerability window is of {} minutes ({})",
                m,
                if m <= 5 { "GOOD!" } else { "acceptable" }
            )
            .as_str(),
        ),
        (0, h, _) => log_info(
            format!(
                "Vulnerability window is of {} hours ({})",
                h,
                if h < 2 {
                    "ok"
                } else if h < 4 {
                    "a bit long"
                } else {
                    "too long, be cautious"
                }
            )
            .as_str(),
        ),
        (d, _, _) => log_info(
            format!(
                "Vulnerability window is of {} day{} (much too long)",
                d,
                if d > 1 { "s" } else { "" }
            )
            .as_str(),
        ),
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

    // Issue download request so a request to download an inexisting file
    // can be reported before we report a missing asfaload index file.
    // The response object is used much lower in the code, after we get
    // the asfaload index file.
    let response = fetch_url(url.clone())
        .await
        .map_err(|e| {
            let error_msg = format!(
                "File to download not found.\nOriginal error: {}{}",
                e,
                e.source()
                    .map(|s| "\n Source: ".to_string() + s.to_string().as_str())
                    .unwrap_or("".to_string())
            );
            anyhow::Error::msg(error_msg)
        })?
        .error_for_status()?;
    let url_path = url
        .path_segments()
        .map(|c| c.map(|s| s.to_owned()).collect::<Vec<_>>())
        .unwrap_or_else(std::vec::Vec::new);

    let file = url_path.last().context("No file found in URL")?.to_owned();
    let dest_file = args.output.unwrap_or(file.clone());
    if (dest_file != "-") & !args.overwrite & (Path::new(&dest_file).exists()) {
        anyhow::bail!(format!(
            "Destination file exists ({}).\nNot overwriting files, please remove it or use the --overwrite flag.",
            dest_file
        ));
    }

    let path = url_path[..url_path.len() - 1].join("/");

    let mut checksum = match args.checksum_source.hash_value {
        // The hash string was passed to the CLI with the flag --hash. We use it and
        // don't look for a file on a server.
        Some(hash) => {
            log_info("Using hash passed as argument");
            Checksum::from_hash(file.as_str(), hash.as_str())?.into_validator(file.as_str())
        }
        // No hash value was passed as argument to the CLI with the --hash flag.
        // This means we need to look for the hash in a file.
        None => {
            if !checksum_flag.asfaload_host
                && checksum_flag.checksum_patterns.is_empty()
                && !checksum_flag.no_asfaload_index
            {
                log_info("Using asfaload index on mirror");
                let (validator, index) =
                    index::validator_and_index_for(&url, args.force_invalid).await?;
                log_info(
                    format!(
                        "Artifact was published on {} and checksums file was mirrored on {}",
                        index.published_on, index.mirrored_on
                    )
                    .as_str(),
                );
                let vuln_window = index.vulnerability_window();
                print_vulnerability_window_info(&vuln_window);
                Some(validator)
            } else {
                log_info("Will for hash in a checksums file");
                // Create a hashmap with the path and file to be used in the templates
                let vars = HashMap::from([
                    ("fullpath".to_string(), url_path.join("/")),
                    ("path".to_string(), path),
                    ("file".to_string(), file.clone()),
                ]);
                // This shouldn't happen:
                envsubst::validate_vars(&vars)
                    .context("unable to validate substitution variables")?;

                log_step(SEARCH, "Looking for checksum files...");
                // Create a stream of checksum downloads
                let checksums_patterns = repo_checksums::CHECKSUMS_FILES
                    .iter()
                    .chain(checksum_flag.checksum_patterns.iter())
                    // It is safe to unwrap as the only possible error is catched by the validate_vars above
                    .map(|tmpl| envsubst::substitute(tmpl, &vars).unwrap())
                    // Build the URL where to get the checksums file.
                    .map(|pattern| {
                        // Helper to build the replace the path of url by the path passed as argument
                        // Template is supposedly a full url
                        if pattern.starts_with("http") {
                            use_pattern_as_url_if_valid_scheme(&url, &pattern)
                        }
                        // Look on our checksums mirrors
                        else if checksum_flag.asfaload_host {
                            let url = update_url_path(&url, &pattern);
                            update_url_asfaload_host(&url)
                        // Template is a path, look on same server as file
                        } else {
                            update_url_path(&url, &pattern)
                        }
                    })
                    .map(|url| Box::pin(fetch_checksum(url, &file)));

                // Select the first checksum file that is found
                match select_ok(checksums_patterns).await {
                    Ok(((checksum, url), _)) => {
                        log_step(
                            FOUND,
                            format!("Checksum file found at {}!", url.host().unwrap()).as_str(),
                        );
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
                }
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
    if dest_file == "-" {
        let mut f = File::open(temp_file_path).await?;
        let mut buffer = [0; 100_000];
        let mut bytes_read = f.read(&mut buffer).await?;
        while bytes_read > 0 {
            std::io::stdout().write_all(&buffer[0..bytes_read]).unwrap();
            bytes_read = f.read(&mut buffer).await?;
        }
        std::io::stdout().flush().unwrap();
    } else {
        fs::rename(temp_file_path, &dest_file)
            .await
            .context(format!("Unable to move the temporary file to {dest_file}"))?;
    }

    Ok(())
}

#[cfg(test)]
mod helpers_tests {
    use asfald::handle_pattern;

    use super::*;

    #[test]
    fn test_update_url_path() {
        let new_path = "/asfald-checksums/v0.0.1";
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
