use assert_cmd::prelude::*; // Add methods on commands
use predicates::path::is_file;
use predicates::prelude::*;
use predicates::str::contains;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use testdir::testdir;

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
// Test -o flag
fn file_with_valid_checksum_o() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfd");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("the_local_file.txt"));
    cmd.arg(url("/valid/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .success()
        .stdout(contains("Checksum file found !"))
        .stdout(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    // Check the original filename is not present
    assert!(!is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
    // Check the file was downloaded in the requested location
    assert!(is_file_pred.eval(Path::new(&dir.join("the_local_file.txt"))));
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
