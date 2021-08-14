use assert_cmd::Command;
#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor2").unwrap();
    cmd.arg("hello").assert().success();
}
