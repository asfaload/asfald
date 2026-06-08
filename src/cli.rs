use clap::Parser;
use std::path::PathBuf;
use url::Url;

use crate::flags::{BackendUrlArgs, ForgeTypeArgs};

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

    /// Print digest without downloading target file
    #[arg(short, long)]
    pub get_hash: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// No output, even no progress bar
    #[arg(short, long)]
    pub quiet: bool,

    #[command(flatten)]
    backend_url_args: BackendUrlArgs,

    #[command(flatten)]
    forge_type_args: ForgeTypeArgs,

    pub url: Url,
}
