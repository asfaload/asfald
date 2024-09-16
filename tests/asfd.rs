use assert_cmd::prelude::*; // Add methods on commands
use predicates::path::is_file;
use predicates::prelude::*;
use predicates::str::contains;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use testdir::testdir;

const HTTP_HOST: &str = "http://localhost";

// Helper to return the furl url of the test file to be downloaded
fn url(path: &str) -> String {
    format!("{HTTP_HOST}:9988{path}")
}

// Helper to return the url of the path for the second test server
fn snd_url(path: &str) -> String {
    format!("{HTTP_HOST}:9989{path}")
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
// Test -p flag
fn file_with_valid_checksum_p_url() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfd");
    // Download the file to our dedicated directory
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    // Specify the url where to get the checksums file
    cmd.arg("-p");
    cmd.arg(snd_url("/remote_checksum/checksums.txt"));
    // Get the file from its server
    cmd.arg(url("/remote_checksum/the_file.txt"));

    cmd.assert()
        .success()
        .stdout(contains("Checksum file found !"))
        .stdout(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    // Check the original filename is not present
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
}

#[test]
// Test -p flag
fn file_with_valid_checksum_p_fullpath() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfd");
    // Download the file to our dedicated directory
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    // Specify the url where to get the checksums file
    cmd.arg("-p");
    cmd.arg("${fullpath}.sha256");
    // Get the file from its server
    cmd.arg(url("/valid_suffix/the_file.txt"));

    cmd.assert()
        .success()
        .stdout(contains("Checksum file found !"))
        .stdout(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    // Check the original filename is not present
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
}

#[test]
// Test -p flag
fn file_with_valid_checksum_p_file_pattern() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfd");
    // Download the file to our dedicated directory
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    // Specify the url where to get the checksums file
    cmd.arg("-p");
    cmd.arg(snd_url("/remote_p_file_pattern/publish/${file}.checksum"));
    // Get the file from its server
    cmd.arg(url("/remote_p_file_pattern/the_file.txt"));

    cmd.assert()
        .success()
        .stdout(contains("Checksum file found !"))
        .stdout(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    // Check the original filename is not present
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
}

#[test]
// Test -p flag
fn file_with_valid_checksum_p_path_pattern() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfd");
    // Download the file to our dedicated directory
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    // Specify the url where to get the checksums file
    cmd.arg("-p");
    cmd.arg(snd_url("/${path}/${file}.checksum"));
    // Get the file from its server
    cmd.arg(url(
        "/remote_p_path_pattern/publish/releases/latest/the_file.txt",
    ));

    cmd.assert()
        .success()
        .stdout(contains("Checksum file found !"))
        .stdout(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    // Check the original filename is not present
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
}

#[test]
// Test -p flag
fn file_p_pattern_with_http() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfd");
    // Download the file to our dedicated directory
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    // Specify the url where to get the checksums file
    cmd.arg("-p");
    cmd.arg("http/checksums.txt");
    // Get the file from its server
    cmd.arg(url("/p_pattern_http/the_file.txt"));

    cmd.assert()
        .success()
        .stdout(contains("Checksum file found !"))
        .stdout(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    // Check the original filename is not present
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
}

#[test]
// Test -q flag
fn file_with_valid_checksum_q() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfd");
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    cmd.arg("-q");
    // Download the file to our dedicated directory
    cmd.arg(url("/valid/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert().success().stdout(predicates::str::is_empty());

    let is_file_pred = is_file();
    // Check the original filename is not present
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
}

#[test]
// Test without checksums file
fn file_without_checksums_file() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfd");
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    // Download the file to our dedicated directory
    cmd.arg(url("/no_checksums_file/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .failure()
        .stdout(contains("Checksum file found !").not())
        .stdout(contains("File\'s checksum is valid !").not())
        .stderr(contains("Unable to fetch checksum file"));

    let is_file_pred = is_file();
    // Check no file was downloaded
    assert!(!is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
}

#[test]
// Test without checksums file and force-absent
fn file_without_checksums_file_but_force_absent() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfd");
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    cmd.arg("-f");
    // Download the file to our dedicated directory
    cmd.arg(url("/no_checksums_file/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .success()
        .stdout(contains("Checksum file found !").not())
        .stdout(contains("File\'s checksum is valid !").not())
        .stdout(contains(
            "Checksum file not found, but continuing due to --force-absent or --force-invalid flag",
        ))
        .stderr(predicates::str::is_empty());

    let is_file_pred = is_file();
    // Check no file was downloaded
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
}

#[test]
// Test without checksums file and force-invalidn, which implies force-absent
fn file_without_checksums_file_but_force_invalid() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfd");
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    cmd.arg("-F");
    // Download the file to our dedicated directory
    cmd.arg(url("/no_checksums_file/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .success()
        .stdout(contains("Checksum file found !").not())
        .stdout(contains("File\'s checksum is valid !").not())
        .stdout(contains(
            "Checksum file not found, but continuing due to --force-absent or --force-invalid flag",
        ))
        .stderr(predicates::str::is_empty());

    let is_file_pred = is_file();
    // Check no file was downloaded
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
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

#[test]
// File downloaded is present in checksums file, but the checksum is different
// With --force-absent: should validate the checksum if it is present
fn file_with_invalid_checksum_force_absent() {
    let mut cmd = Command::new("target/debug/asfd");
    cmd.arg(url("/invalid_checksum/the_file.txt"));
    cmd.arg("-f");
    cmd.assert()
        .failure()
        .stdout(contains("Checksum file found !"))
        .stdout(contains("File\'s checksum is invalid !"));
}

#[test]
// File downloaded is present in checksums file, but the checksum is different
// With --force-absent: should validate the checksum if it is present
fn file_with_invalid_checksum_force_invalid() {
    let mut cmd = Command::new("target/debug/asfd");
    cmd.arg(url("/invalid_checksum/the_file.txt"));
    cmd.arg("-F");
    cmd.assert()
        .success()
        .stdout(contains("Checksum file found !"))
        .stdout(contains("File\'s checksum is invalid !"))
        .stdout(contains("⚠️⚠️ WARNING: this is insecure, and still downloads file with a checksum present, but invalid! ⚠️⚠️"));
}

#[test]
// Test successful download without any flag
fn file_with_path_and_valid_checksum() {
    let mut cmd = Command::new("target/debug/asfd");
    cmd.arg(url("/checksums_with_path/the_file.txt"));
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
