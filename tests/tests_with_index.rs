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

#[test]
fn file_with_invalid_chksum_on_mirror() {
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("invalid_chksum_on_mirror"));
    cmd.arg(url(
        "/asfaload/asfald/releases/download/v0.1.0/invalid_chksum_on_mirror",
    ));
    cmd.assert().failure().stderr(contains(
        "Checksum found on mirror is different from checksum found in release.",
    ));

    let is_file_pred = is_file();
    assert!(!is_file_pred.eval(Path::new(&dir.join("invalid_chksum_on_mirror"))));
    let _ = std::fs::remove_dir(dir);

    // When we force download despite invalid checksums, we still download the file
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("--force-invalid");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("saved_file"));
    cmd.arg(url(
        "/asfaload/asfald/releases/download/v0.1.0/invalid_chksum_on_mirror",
    ));
    cmd.assert()
        .success()
        .stderr(contains("File's checksum is invalid !"))
        .stderr(contains("but continuing due to --force-invalid flag"));

    let is_file_pred = is_file();
    assert!(is_file_pred.eval(Path::new(&dir.join("saved_file"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
fn file_with_updated_chksum_in_release() {
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("saved_file"));
    cmd.arg(url(
        "/asfaload/asfald/releases/download/v0.1.0/chksum_modified_in_release",
    ));
    cmd.assert().failure().stderr(contains(
        "Checksum found on mirror is different from checksum found in release.",
    ));

    let is_file_pred = is_file();
    assert!(!is_file_pred.eval(Path::new(&dir.join("saved_file"))));
    let _ = std::fs::remove_dir(dir);

    // When we force download despite invalid checksums, we still download the file
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("--force-invalid");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("saved_file"));
    cmd.arg(url(
        "/asfaload/asfald/releases/download/v0.1.0/chksum_modified_in_release",
    ));
    cmd.assert().success().stderr(contains(
        "Checksum found in release is different, but continuing as --force-invalid flag found",
    ));

    let is_file_pred = is_file();
    assert!(is_file_pred.eval(Path::new(&dir.join("saved_file"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
fn file_with_updated_sha256_in_release() {
    // For this download, we use the best checksum available, which is sha512, and we do
    // not look to sha256. As for sha512 checksums match on mirror and in release, we proceed
    // and the download is sucessful
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("saved_file"));
    cmd.arg(url(
        "/asfaload/asfald/releases/download/v0.1.0/updated_sha256_in_release",
    ));
    cmd.assert()
        .success()
        .stderr(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    assert!(is_file_pred.eval(Path::new(&dir.join("saved_file"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
fn file_ok_on_server_not_in_index() {
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("saved_file"));
    cmd.arg(url(
        "/asfaload/asfald/releases/download/v0.1.0/on_server_checksums_but_not_index",
    ));
    cmd.assert()
        .failure()
        .stderr(contains("Didn't find checksum for file in index file"));

    let is_file_pred = is_file();
    assert!(!is_file_pred.eval(Path::new(&dir.join("saved_file"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
fn file_without_index() {
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("saved_file"));
    cmd.arg(url(
        "/asfaload/asfald/releases/download/release-without-index/file_without_index",
    ));
    cmd.assert().failure().stderr(contains(
        "Problem getting asfalod index file, is the project tracked by asfaload?",
    ));

    let is_file_pred = is_file();
    assert!(!is_file_pred.eval(Path::new(&dir.join("saved_file"))));
    let _ = std::fs::remove_dir(dir);
}
