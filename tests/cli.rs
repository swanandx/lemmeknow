use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*; // Used for files
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn identify_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("127.0.0.1\n13-08-1987\nidk what this is lol")?;

    let mut cmd = Command::cargo_bin("lemmeknow")?;
    cmd.arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Internet Protocol"));

    Ok(())
}


#[test]
fn identify_url() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lemmeknow")?;

    cmd.arg("https://github.com/swanandx/lemmeknow");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Uniform Resource Locator"));

    Ok(())
}

#[test]
fn failed_to_identify() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lemmeknow")?;

    cmd.arg("afsjdla");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No Possible Identifications"));

    Ok(())
}

#[test]
fn check_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lemmeknow")?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("lemmeknow"));

    Ok(())
}
