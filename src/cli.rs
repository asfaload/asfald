use clap::Parser;
use std::path::PathBuf;
use url::Url;

#[derive(Parser, Debug)]
#[command(
    name = "asfald",
    about = "Downloads files from GitHub releases with hash verification",
    version,
    author
)]
pub struct Cli {
    /// Output file path
    #[arg(short, long, value_name = "PATH")]
    pub output: Option<PathBuf>,

    /// GitHub API token (can also be set via GITHUB_API_KEY env var)
    #[arg(short, long, env = "GITHUB_API_KEY")]
    pub token: Option<String>,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// No output, even no progress bar
    #[arg(short, long)]
    pub quiet: bool,

    pub url: Url,
}
