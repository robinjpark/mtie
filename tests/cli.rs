use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn test_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mtie")?;

    cmd.arg("--input").arg("test/file/does/not/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn test_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mtie")?;

    cmd.arg("--version");
    cmd.assert().success();

    Ok(())
}

#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mtie")?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("USAGE"))
        .stdout(predicate::str::contains("Robin Park"))
        .stdout(predicate::str::contains("robin.j.park@gmail.com"))
        .stdout(predicate::str::contains("Calculates MTIE from a series of TIE input data."));

    Ok(())
}

