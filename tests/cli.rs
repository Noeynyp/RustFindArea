use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;
#[test]
fn area() -> TestResult {
let expected = "area: 28.274399\n";
let mut cmd = Command::cargo_bin("area")?;
cmd.arg("3").assert().success().stdout(expected);
Ok(())
}