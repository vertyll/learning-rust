use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn pass_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo-clap")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
    Ok(())
}