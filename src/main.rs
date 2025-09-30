use asfald::{cli::Cli, downloader::Downloader, error::Result};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let downloader = Downloader::new();
    let output_path = cli.output.as_deref();

    match downloader
        .download_and_verify(cli.url, output_path, cli.quiet)
        .await
    {
        Ok(result) => {
            if cli.verbose {
                println!("Successfully downloaded and verified file:");
                println!("  Path: {}", result.path.display());
                println!("  Size: {} bytes", result.size);
                println!("  Algorithm: {}", result.algorithm);
                println!("  Hash: {}", result.hash);
            }
        }
        Err(e) => {
            if !cli.quiet {
                eprintln!("Error: {}", e);
            }
            std::process::exit(1);
        }
    }

    Ok(())
}
