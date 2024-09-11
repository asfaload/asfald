use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command;

const HTTP_SERVER: &str = "http://localhost:9988";

// Helper to return the furl url of the test file to be downloaded
fn url(path: &str) -> String {
    format!("{HTTP_SERVER}{path}")
}

#[test]
fn file_with_valid_checksum() {
    let mut cmd = Command::new("target/debug/asfd");
    cmd.arg(url("/valid/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert().success();
}

#[test]
fn file_with_invalid_checksum() {
    let mut cmd = Command::new("target/debug/asfd");
    cmd.arg(url("/invalid_checksum/the_file.txt"));
    cmd.assert().failure();
}
