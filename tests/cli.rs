use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult{
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() -> TestResult{
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("HELLO WORLD")
        .assert()
        .success()
        .stdout(predicate::str::contains("HELLO WORLD"));
    Ok(())
}