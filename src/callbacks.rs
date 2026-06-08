use std::io::Write;

use client_lib::{DownloadCallbacks, constants::ONE_MEGABYTE};

pub fn basic() -> DownloadCallbacks {
    DownloadCallbacks::default()
        .with_starting(|args| {
            println!("Starting download: {}", args.file_url);
        })
        .with_signers_downloaded(|args| {
            println!("✓ Downloaded signers file ({} bytes)", args.bytes);
        })
        .with_index_downloaded(|args| {
            println!("✓ Downloaded index file ({} bytes)", args.bytes);
        })
        .with_signatures_downloaded(|args| {
            println!("✓ Downloaded signatures file ({} bytes)", args.bytes);
        })
        .with_signatures_verified(|args| {
            if args.invalid_count > 0 {
                println!("⚠ Warning: {} invalid signature(s)", args.invalid_count);
            }
            println!(
                "✓ Signatures verified successfully ({} valid)",
                args.valid_count
            );
        })
        .with_revocation_detected(|args| {
            eprintln!("This file has been revoked.");
            eprintln!("  Revoked at: {}", args.timestamp);
            eprintln!("  Revoked by: {}", args.initiator);
        })
        .with_signers_chain_verified(|args| {
            println!(
                "✓ Signers chain history verified ({} entries)",
                args.entries_count
            );
        })
        .with_signers_chain_failed(|args| {
            eprintln!("✗ Signers chain verification failed: {}", args.reason);
        })
        .with_file_hash_verified(|args| {
            println!("✓ File hash verified ({})", args.algorithm);
        })
        .with_file_download_started(|args| {
            println!("Downloading {}", args.filename);
            if let Some(size) = args.total_bytes {
                println!("  Size: {:.2} MB", size as f64 / ONE_MEGABYTE as f64);
            }
        })
        .with_file_download_progress(|args| {
            if let Some(total) = args.total_bytes {
                let percent = (args.bytes_downloaded as f64 / total as f64) * 100.0;
                print!(
                    "\rProgress: {:.1}% ({:.2} MB / {:.2} MB)",
                    percent,
                    args.bytes_downloaded as f64 / ONE_MEGABYTE as f64,
                    total as f64 / ONE_MEGABYTE as f64
                );
            } else {
                print!(
                    "\rProgress: {:.2} MB",
                    args.bytes_downloaded as f64 / ONE_MEGABYTE as f64
                );
            }
            let _ = std::io::stdout().flush();
        })
        .with_file_download_completed(|args| {
            println!(); // New line after progress
            println!(
                "✓ Download complete ({:.2} MB)",
                args.bytes_downloaded as f64 / ONE_MEGABYTE as f64
            );
        })
        .with_file_saved(|args| {
            println!("✓ File saved to: {}", args.path.display());
        })
        .with_completed(|args| {
            println!(
                "✓ All done! Verified {} signature(s)",
                args.result.signatures_verified
            );
        })
}
