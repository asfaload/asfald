use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use predicates::str::contains;
use std::process::Command;

const HTTP_SERVER: &str = "http://localhost:9988";

// Helper to return the furl url of the test file to be downloaded
fn url(path: &str) -> String {
    format!("{HTTP_SERVER}{path}")
}

#[test]
// Test successful download without any flag
fn file_with_valid_checksum() {
    let mut cmd = Command::new("target/debug/asfd");
    cmd.arg(url("/valid/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .success()
        .stdout(contains("Checksum file found !"))
        .stdout(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    assert!(is_file_pred.eval(Path::new("./the_file.txt")));
    let _ = std::fs::remove_file("./the_file.txt");
}

#[test]
// File downloaded is present in checksums file, but the checksum is different
fn file_with_invalid_checksum() {
    let mut cmd = Command::new("target/debug/asfd");
    cmd.arg(url("/invalid_checksum/the_file.txt"));
    cmd.assert()
        .failure()
        .stdout(contains("Checksum file found !"))
        .stdout(contains("File\'s checksum is invalid !"));
}
