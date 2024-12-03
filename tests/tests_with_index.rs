// These tests use the index files on a mirror.
// For these tests we run two mirrors on localhost, on port 9898 and 9899
// File repos are on localhost port 9988 and 9989
use std::{
    path::{Path, PathBuf},
    process::Command,
};

use assert_cmd::prelude::*; // Add methods on commands
use predicates::{path::is_file, prelude::*, str::contains};
use testdir::testdir;

const HTTP_HOST: &str = "http://localhost";

// Helper to return the furl ull of the test file to be downloaded
fn url(path: &str) -> String {
    format!("{HTTP_HOST}:9988{path}")
}

#[test]
fn file_with_valid_index_entry() {
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("f01"));
    cmd.arg(url("/asfaload/asfald/releases/download/v0.1.0/f01"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .success()
        .stderr(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    assert!(is_file_pred.eval(Path::new(&dir.join("f01"))));
    let _ = std::fs::remove_dir(dir);
}
