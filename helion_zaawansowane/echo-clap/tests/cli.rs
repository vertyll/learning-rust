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

#[test]
fn check_new_line() -> TestResult {
    // let expected = fs::read_to_string("tests/expected/new_line")?;
    // let mut cmd = Command::cargo_bin("echo-clap")?;
    //
    // cmd.args(&["Mikołaj", "Dom", "Pałac"])
    //     .assert()
    //     .success()
    //     .stdout(expected);
    //
    // Ok(())

    let args = &["Mikołaj", "Dom", "Pałac"];
    let file_path = "tests/expected/new_line";

    run_test(args, file_path)
}

#[test]
fn check_no_new_line() -> TestResult {
    // let expected = fs::read_to_string("tests/expected/no_new_line")?;
    // let mut cmd = Command::cargo_bin("echo-clap")?;
    //
    // cmd.args(&["-n", "Mikołaj", "Dom", "Pałac"])
    //     .assert()
    //     .success()
    //     .stdout(expected);
    //
    // Ok(())

    let args = &["-n", "Mikołaj", "Dom", "Pałac"];
    let file_path = "tests/expected/no_new_line";

    run_test(args, file_path)
}

fn run_test(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echo-clap")?;

    cmd.args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}