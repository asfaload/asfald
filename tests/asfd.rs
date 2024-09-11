use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command;

#[test]
fn file_with_valid_checksum() {
    let mut cmd = Command::new("target/debug/asfd");
    cmd.arg("http://localhost:3030/valid/the_file.txt");
    cmd.spawn().unwrap();
    cmd.assert().success();
}

#[test]
fn file_with_invalid_checksum() {
    let mut cmd = Command::new("target/debug/asfd");
    cmd.arg("http://localhost:3030/invalid_checksum/the_file.txt");
    cmd.spawn().unwrap();
    cmd.assert().failure();
}
