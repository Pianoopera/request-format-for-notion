use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;


#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("rf-notion").unwrap();
    cmd.arg("--type").arg("databases");
    cmd.assert().success();
}

// #[test]
// fn runs_curl() -> TestResult {

//     let expected = fs::read_to_string("tests/expected/databases2.txt")?;
//     // notion api の curl コマンドを作成する
//     Command::cargo_bin("rf-notion")
//         .unwrap()
//         .args(&["--type", "databases"])
//         .assert()
//         .success()
//         .stdout(expected);
//     Ok(())
// }

fn runs_output(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    
    // notion api の curl コマンドを作成する
    Command::cargo_bin("rf-notion")
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn databases1() -> TestResult {
    runs_output(&["--type", "databases"], "tests/expected/databases1.txt")
}
