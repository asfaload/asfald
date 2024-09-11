use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command;

#[test]
fn it_works() {
    let mut cmd = Command::new("/usr/bin/curl");
    cmd.arg("http://localhost:3030/data/valid/the_file.txt");
    println!("will assert success");
    cmd.spawn().unwrap();
    cmd.assert().success();
}
