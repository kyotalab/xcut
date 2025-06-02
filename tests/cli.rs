use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_basic_cut_columns() {
    let mut cmd = Command::cargo_bin("xcut").unwrap();
    cmd.args(&[
        "--input",
        "tests/fixtures/input1.txt",
        "--cols",
        "3,4",
        "--delim",
        " ",
        "--max-split",
        "4",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("INFO Connection established"));
}

#[test]
fn test_filter_with_regex_match() {
    let mut cmd = Command::cargo_bin("xcut").unwrap();
    cmd.args(&[
        "--input",
        "tests/fixtures/input1.txt",
        "--cols",
        "3",
        "--filter",
        r#"col(3) =~ "ERROR""#,
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("ERROR"));
}

#[test]
fn test_filter_with_regex_not_match() {
    let mut cmd = Command::cargo_bin("xcut").unwrap();
    cmd.args(&[
        "--input",
        "tests/fixtures/input1.txt",
        "--cols",
        "3",
        "--filter",
        r#"col(3) !~ "INFO""#,
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("ERROR"))
    .stdout(predicate::str::contains("WARN"));
}
