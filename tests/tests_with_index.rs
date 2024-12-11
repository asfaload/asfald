// These tests use the index files on a mirror.
// For these tests we run two mirrors on localhost, on port 9898 and 9899
// File repos are on localhost port 9988 and 9989
use std::{
    fs::File,
    path::{Path, PathBuf},
    process::{Command, Stdio},
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

    // Check the checksum of the downloaded file so we are sure that
    // when moving the file from its temporary location we don't alter it
    let mut cmd = Command::new("/usr/bin/sha256sum");
    cmd.current_dir(dir.as_path());
    cmd.arg("f01");
    cmd.assert().success().stdout(contains(
        "972612a7a8370b797bc1d7736c01ff42b3e1ec23ec1ff6f0f1020feb6047e0d9",
    ));

    let is_file_pred = is_file();
    assert!(is_file_pred.eval(Path::new(&dir.join("f01"))));
    let _ = std::fs::remove_dir(dir);

    // Pipe to stdout
    // --------------
    // asfald pipes to stdout, and we save this to a file.
    let dir: PathBuf = testdir!();
    let output_path = dir.join("captured_output");
    let output_file = File::create(output_path).expect("failed to open log");
    #[allow(clippy::zombie_processes)]
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o")
        // Download the file to our dedicated directory
        .arg("-")
        .arg(url("/asfaload/asfald/releases/download/v0.1.0/f01"))
        .stdout(output_file);
    cmd.assert().success();

    let is_file_pred = is_file();
    assert!(is_file_pred.eval(Path::new(&dir.join("captured_output"))));

    // Check the captured output is as expected
    let mut cmd = Command::new("/usr/bin/sha256sum");
    cmd.arg(Path::new(&dir.join("captured_output")));
    cmd.assert().success().stdout(contains(
        "972612a7a8370b797bc1d7736c01ff42b3e1ec23ec1ff6f0f1020feb6047e0d9",
    ));

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

    // Similar test: the file in the release has been renamed after mirror taken
    // The mirror has the old file name and does not find the new name
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("saved_file"));
    cmd.arg(url(
        "/asfaload/asfald/releases/download/v0.1.0/renamed_after_index",
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
        "Problem getting asfaload index file, is the project tracked by asfaload?",
    ));

    let is_file_pred = is_file();
    assert!(!is_file_pred.eval(Path::new(&dir.join("saved_file"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
fn install_sh_tests() {
    // Save on disk
    // ------------
    // Start with a comprehensive save on disk test to set the stage
    let dir: PathBuf = testdir!();
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("install.sh"));
    cmd.arg(url("/asfaload/asfald/releases/download/v0.1.0/install.sh"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .success()
        .stderr(contains("File\'s checksum is valid !"));

    // Check the checksum of the downloaded file after its move from the
    // temp location
    let mut cmd = Command::new("/usr/bin/sha256sum");
    cmd.current_dir(dir.as_path());
    cmd.arg("install.sh");
    cmd.assert().success().stdout(contains(
        "9aad36aa9acad2311d606e5927a4be14e8899a49bd3279f1597c3941404601e3",
    ));
    let is_file_pred = is_file();
    assert!(is_file_pred.eval(Path::new(&dir.join("install.sh"))));
    let _ = std::fs::remove_dir(dir);

    // Pipe to stdout
    // --------------
    let dir: PathBuf = testdir!();
    #[allow(clippy::zombie_processes)]
    let cmd = Command::new("target/debug/asfald")
        .arg("-o")
        // Download the file to our dedicated directory
        .arg("-")
        .arg(url("/asfaload/asfald/releases/download/v0.1.0/install.sh"))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start asfald process");

    let asfald_out = cmd.stdout.expect("Failed to open asfald stdout");
    let _bash = Command::new("/bin/bash")
        .stdin(Stdio::from(asfald_out))
        .assert()
        .success()
        .stdout(contains("The downloaded script is executed succesfully!"));

    let is_file_pred = is_file();
    assert!(!is_file_pred.eval(Path::new(&dir.join("saved_file"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
fn binary_pipe() {
    let dir: PathBuf = testdir!();
    #[allow(clippy::zombie_processes)]
    let cmd = Command::new("target/debug/asfald")
        .arg("-q")
        .arg("-o")
        // Download the file to our dedicated directory
        .arg("-")
        .arg(url("/asfaload/asfald/releases/download/v0.1.0/archive.tgz"))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start asfald process");

    let asfald_out = cmd.stdout.expect("Failed to open asfald stdout");
    #[allow(clippy::zombie_processes)]
    let gunzip = Command::new("/usr/bin/gunzip")
        .arg("-c")
        .stdin(Stdio::from(asfald_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start gunzip process");
    let gunzip_out = gunzip.stdout.expect("Failed to open gunzip stdout");

    let _tar = Command::new("/usr/bin/tar")
        .arg("-C")
        .arg(&dir)
        .arg("-xf")
        .arg("-")
        .stdin(Stdio::from(gunzip_out))
        .assert()
        .success();
    let is_file_pred = is_file();
    assert!(is_file_pred.eval(Path::new(&dir.join("f09"))));
    assert!(is_file_pred.eval(Path::new(&dir.join("f10"))));
    let _ = std::fs::remove_dir(dir);
}

#[test]
fn not_overwriting() {
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

    // Downloading the same file to the same location is rejected
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("f01"));
    cmd.arg(url("/asfaload/asfald/releases/download/v0.1.0/f01"));
    // spawn will display the output of the command
    //cmd.spawn().unwrap();
    cmd.assert()
        .failure()
        .stderr(contains(format!(
            "Destination file exists ({}).",
            dir.join("f01").to_str().unwrap()
        )))
        .stderr(contains(
            "Not overwriting files, please remove it or use the --overwrite flag.",
        ));

    // Except if the flag ovrewrite is passed
    let mut cmd = Command::new("target/debug/asfald");
    cmd.arg("-o");
    // Download the file to our dedicated directory
    cmd.arg(dir.join("f01"));
    cmd.arg("--overwrite");
    cmd.arg(url("/asfaload/asfald/releases/download/v0.1.0/f01"));
    cmd.assert()
        .success()
        .stderr(contains("File\'s checksum is valid !"))
        .stderr(contains("Not overwriting files, please remove it.").not());
    let _ = std::fs::remove_dir(dir);
}
