use std::{
    path::{Path, PathBuf},
    process::Command,
};

use assert_cmd::prelude::*; // Add methods on commands
use predicates::{path::is_file, prelude::*, str::contains};
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
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-I");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("the_local_file.txt"));
    cmd.arg(url("/valid/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .success()
        .stderr(contains("Checksum file found at localhost!"))
        .stderr(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    assert!(is_file_pred.eval(Path::new(&dir.join("the_local_file.txt"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
// Test -o flag
fn file_with_valid_checksum_o() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-I");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("the_local_file.txt"));
    cmd.arg(url("/valid/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .success()
        .stderr(contains("Checksum file found at localhost!"))
        .stderr(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    // Check the original filename is not present
    assert!(!is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
    // Check the file was downloaded in the requested location
    assert!(is_file_pred.eval(Path::new(&dir.join("the_local_file.txt"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
// Test -p flag
fn file_with_valid_checksum_p_url() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
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
        .stderr(contains("Checksum file found at localhost!"))
        .stderr(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    // Check the original filename is not present
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
// Test -p flag
fn file_with_valid_checksum_p_fullpath() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
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
        .stderr(contains("Checksum file found at localhost!"))
        .stderr(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    // Check the original filename is not present
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
// Test -p flag
fn file_with_valid_checksum_p_file_pattern() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
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
        .stderr(contains("Checksum file found at localhost!"))
        .stderr(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    // Check the original filename is not present
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
// Test -p flag
fn file_with_valid_checksum_p_path_pattern() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
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
        .stderr(contains("Checksum file found at localhost!"))
        .stderr(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    // Check the original filename is not present
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
// Test -p flag
fn file_p_pattern_with_http() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
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
        .stderr(contains("Checksum file found at localhost!"))
        .stderr(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    // Check the original filename is not present
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
// Test -q flag
fn file_with_valid_checksum_q() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    cmd.arg("-I");
    cmd.arg("-q");
    // Download the file to our dedicated directory
    cmd.arg(url("/valid/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert().success();

    let is_file_pred = is_file();
    // Check the original filename is not present
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
// Test without checksums file
fn file_without_checksums_file() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-I");
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    // Download the file to our dedicated directory
    cmd.arg(url("/no_checksums_file/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .failure()
        .stderr(contains("Checksum file found at localhost!").not())
        .stderr(contains("File\'s checksum is valid !").not())
        .stderr(contains("Unable to fetch checksum file"));

    let is_file_pred = is_file();
    // Check no file was downloaded
    assert!(!is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
// Test without checksums file and force-absent
fn file_without_checksums_file_but_force_absent() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-I");
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    cmd.arg("-f");
    // Download the file to our dedicated directory
    cmd.arg(url("/no_checksums_file/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .success()
        .stderr(contains("Checksum file found at localhost!").not())
        .stderr(contains("File\'s checksum is valid !").not())
        .stderr(contains(
            "Checksum file not found, but continuing due to --force-absent or --force-invalid flag",
        ));

    let is_file_pred = is_file();
    // Check no file was downloaded
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
// Test without checksums file and force-invalidn, which implies force-absent
fn file_without_checksums_file_but_force_invalid() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-I");
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    cmd.arg("-F");
    // Download the file to our dedicated directory
    cmd.arg(url("/no_checksums_file/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .success()
        .stderr(contains("Checksum file found at localhost!").not())
        .stderr(contains("File\'s checksum is valid !").not())
        .stderr(contains(
            "Checksum file not found, but continuing due to --force-absent or --force-invalid flag",
        ));

    let is_file_pred = is_file();
    // Check no file was downloaded
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
// File downloaded is present in checksums file, but the checksum is different
fn file_with_invalid_checksum() {
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-I");
    cmd.arg(url("/invalid_checksum/the_file.txt"));
    cmd.assert()
        .failure()
        .stderr(contains("Checksum file found at localhost!"))
        .stderr(contains("File\'s checksum is invalid !"));
}

#[test]
// File downloaded is present in checksums file, but the checksum is different
// With --force-absent: should validate the checksum if it is present
fn file_with_invalid_checksum_force_absent() {
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-I");
    cmd.arg(url("/invalid_checksum/the_file.txt"));
    cmd.arg("-f");
    cmd.assert()
        .failure()
        .stderr(contains("Checksum file found at localhost!"))
        .stderr(contains("File\'s checksum is invalid !"));
}

#[test]
// File downloaded is present in checksums file, but the checksum is different
// With --force-absent: should validate the checksum if it is present
fn file_with_invalid_checksum_force_invalid() {
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-I");
    cmd.arg(url("/invalid_checksum/the_file.txt"));
    cmd.arg("-F");
    cmd.assert()
        .success()
        .stderr(contains("Checksum file found at localhost!"))
        .stderr(contains("File\'s checksum is invalid !"))
        .stderr(contains("⚠️⚠️ WARNING: this is insecure, and still downloads file with a checksum present, but invalid! ⚠️⚠️"));
}

#[test]
// Test successful download without any flag
fn file_with_path_and_valid_checksum() {
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-I");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("the_local_file.txt"));
    cmd.arg(url("/checksums_with_path/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .success()
        .stderr(contains("Checksum file found at localhost!"))
        .stderr(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    assert!(is_file_pred.eval(Path::new(&dir.join("the_local_file.txt"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
// Test successful download without any flag
fn file_with_binary_indicator() {
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-I");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("the_local_file.txt"));
    cmd.arg(url("/checksums_with_binary_indicator/the_file.txt"));
    cmd.assert()
        .success()
        .stderr(contains("Checksum file found at localhost!"))
        .stderr(contains("File\'s checksum is valid !"));

    let is_file_pred = is_file();
    assert!(is_file_pred.eval(Path::new(&dir.join("the_local_file.txt"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
// Test without checksums file
fn cli_with_hash_flag() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();

    // Test with the right hash value
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    cmd.arg("--hash");
    cmd.arg("5551b7a5370158efdf4158456feb85f310b3233bb7e71253e3b020fd465027ab");
    // Download the file to our dedicated directory
    cmd.arg(url("/no_checksums_file/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .success()
        .stderr(contains("Checksum file found at localhost!").not())
        .stderr(contains("Using hash passed as argument"))
        .stderr(contains("File\'s checksum is valid !"))
        .stderr(contains("Unable to fetch checksum file").not());

    let is_file_pred = is_file();
    // Check file was downloaded
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
    let _ = std::fs::remove_file(Path::new(&dir.join("the_file.txt")));

    // Test with a wrong hash value on the CLI: should be rejected
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    cmd.arg("--hash");
    cmd.arg("000000a5370158efdf4158456feb85f310b3233bb7e71253e3b020fd46000000");
    // Download the file to our dedicated directory
    cmd.arg(url("/no_checksums_file/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .failure()
        .stderr(contains("Checksum file found at localhost!").not())
        .stderr(contains("Using hash passed as argument"))
        .stderr(contains("File\'s checksum is invalid !"))
        .stderr(contains("Unable to fetch checksum file").not());

    // Check no file was downloaded
    assert!(!is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));

    // Test with a checksum file on the server with the wrong value, but pass the right value on the
    // CLI
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    cmd.arg(url("/invalid_checksum/the_file.txt"));
    cmd.arg("--hash");
    cmd.arg("5551b7a5370158efdf4158456feb85f310b3233bb7e71253e3b020fd465027ab");
    cmd.assert()
        .success()
        .stderr(contains("Using hash passed as argument"))
        .stderr(contains("Checksum file found at localhost!").not())
        .stderr(contains("File\'s checksum is valid !"));
    // Check file was downloaded
    assert!(is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
    let _ = std::fs::remove_file(Path::new(&dir.join("the_file.txt")));
    //
    // Test with a wrong hash value on the CLI while the server has a checksums file
    // with the right hash. This should be rejected.
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    cmd.arg("--hash");
    cmd.arg("000000a5370158efdf4158456feb85f310b3233bb7e71253e3b020fd46000000");
    // Download the file to our dedicated directory
    cmd.arg(url("/valid/the_file.txt"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .failure()
        .stderr(contains("Checksum file found at localhost!").not())
        .stderr(contains("Using hash passed as argument"))
        .stderr(contains("File\'s checksum is invalid !"))
        .stderr(contains("Unable to fetch checksum file").not());

    // Check no file was downloaded
    assert!(!is_file_pred.eval(Path::new(&dir.join("the_file.txt"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
// Test without checksums file
fn cli_with_hash_and_p_flags() {
    // Create out dedicated directory
    let dir: PathBuf = testdir!();

    // Test with the right hash value
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    cmd.arg(dir.join("the_file.txt"));
    cmd.arg("--hash");
    cmd.arg("5551b7a5370158efdf4158456feb85f310b3233bb7e71253e3b020fd465027ab");
    cmd.arg("-p");
    cmd.arg("http://example.com/checksum.txt");
    // Download the file to our dedicated directory
    cmd.arg(url("/no_checksums_file/the_file.txt"));
    cmd.assert().failure().stderr(contains(
        "error: the argument \'--hash <HASH>\' cannot be used with \'--pattern <TEMPLATE>\'",
    ));
    let _ = std::fs::remove_dir(dir);
}
