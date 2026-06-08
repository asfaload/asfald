use clap::Parser;
use std::path::PathBuf;
use url::Url;

use crate::flags::{BackendUrlArgs, ForgeType, ForgeTypeArgs};

/// Default Asfaload backend API URL, used when `--backend-url` is not given.
pub const DEFAULT_BACKEND: &str = "https://backend.asfaload.com";

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

    /// Allow falling back to GitHub's published release digest when Asfaload
    /// protection is unavailable. Weaker verification — opt-in only.
    #[arg(long)]
    pub github_fallback: bool,

    #[command(flatten)]
    backend_url_args: BackendUrlArgs,

    #[command(flatten)]
    forge_type_args: ForgeTypeArgs,

    pub url: Url,
}

impl Cli {
    /// Backend API URL, falling back to `DEFAULT_BACKEND` when unset.
    pub fn backend_url(&self) -> &str {
        self.backend_url_args
            .backend_url
            .as_deref()
            .unwrap_or(DEFAULT_BACKEND)
    }

    /// Forge type override as the string client-lib expects, if any.
    pub fn forge_type(&self) -> Option<&str> {
        self.forge_type_args
            .forge_type
            .as_ref()
            .map(ForgeType::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const URL: &str = "https://github.com/o/r/releases/download/v1/f.tar.gz";

    #[test]
    fn backend_url_defaults_to_constant() {
        let cli = Cli::parse_from(["asfald", URL]);
        assert_eq!(cli.backend_url(), DEFAULT_BACKEND);
    }

    #[test]
    fn backend_url_uses_override() {
        let cli = Cli::parse_from(["asfald", "-u", "https://my.backend", URL]);
        assert_eq!(cli.backend_url(), "https://my.backend");
    }

    #[test]
    fn forge_type_maps_to_str() {
        let cli = Cli::parse_from(["asfald", "-F", "github", URL]);
        assert_eq!(cli.forge_type(), Some("github"));
    }

    #[test]
    fn forge_type_defaults_to_none() {
        let cli = Cli::parse_from(["asfald", URL]);
        assert_eq!(cli.forge_type(), None);
    }

    #[test]
    fn github_fallback_defaults_to_false() {
        let cli = Cli::parse_from(["asfald", URL]);
        assert!(!cli.github_fallback);
    }

    #[test]
    fn github_fallback_enabled_by_flag() {
        let cli = Cli::parse_from(["asfald", "--github-fallback", URL]);
        assert!(cli.github_fallback);
    }
}
