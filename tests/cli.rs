use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}
