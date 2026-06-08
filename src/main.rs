use asfald::{cli::Cli, downloader::Downloader, error::Result};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let downloader = Downloader::new();
    let output_path = cli.output.as_deref();

    if cli.get_hash {
        match downloader.get_hash_for_url(cli.url).await {
            Ok(digest) => println!("{}", digest),
            Err(e) => {
                if !cli.quiet {
                    eprintln!("Error: {}", e);
                }
                std::process::exit(1);
            }
        }
    } else {
        let backend = cli.backend_url();
        let forge_type = cli.forge_type();
        match downloader
            .download(
                cli.url.clone(),
                output_path,
                backend,
                forge_type,
                cli.github_fallback,
                cli.quiet,
            )
            .await
        {
            Ok(()) => {}
            Err(e) => {
                if !cli.quiet {
                    eprintln!("Error: {}", e);
                }
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
