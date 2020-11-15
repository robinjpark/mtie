use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::io::Write;
use std::fs::File;
use std::process::Command; // Run programs
use tempfile::tempdir;

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

#[test]
fn test_mtie_from_file() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempdir()?;
    let tmp_file_path = tmp_dir.path().join("tie");
    let mut tmp_file = File::create(&tmp_file_path)?;
    writeln!(tmp_file, "1.0")?;
    writeln!(tmp_file, "2.1")?;
    writeln!(tmp_file, "3.2")?;

    let mut cmd = Command::cargo_bin("mtie")?;
    cmd.arg("--input")
        .arg(tmp_file_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("1.1"))
        .stdout(predicate::str::contains("2.2"));

    Ok(())
}

#[test]
fn test_bad_data_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempdir()?;
    let tmp_file_path = tmp_dir.path().join("tie");
    let mut tmp_file = File::create(&tmp_file_path)?;
    writeln!(tmp_file, "1.0")?;
    writeln!(tmp_file, "this_is_not_a_number")?;
    writeln!(tmp_file, "2.1")?;

    let mut cmd = Command::cargo_bin("mtie")?;
    cmd.arg("--input")
        .arg(tmp_file_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("1.1"))
        .stderr(predicate::str::contains("Ignoring line 2 'this_is_not_a_number': it does not contain a valid number"));

    Ok(())
}

#[test]
fn test_comments_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempdir()?;
    let tmp_file_path = tmp_dir.path().join("tie");
    let mut tmp_file = File::create(&tmp_file_path)?;
    writeln!(tmp_file, "1.0")?;
    writeln!(tmp_file, "# This is a comment")?;
    writeln!(tmp_file, "2.1")?;

    let mut cmd = Command::cargo_bin("mtie")?;
    cmd.arg("--input")
        .arg(tmp_file_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("1.1"))
        .stderr(predicate::str::is_empty());

    Ok(())
}
