use assert_cmd::Command;

#[cfg(not(windows))]
#[test]
fn test_execute_command_without_path() {
    let assert = Command::cargo_bin("executor")
        .expect("Failed to find binary")
        .current_dir("tests/test_data")
        .args(["execute", "--fname", "script.sh", "--depth", "1"])
        .assert();

    assert.success();
}

// with --path and nested
#[cfg(not(windows))]
#[test]
fn test_execute_command_nested_without_depth() {
    let assert = Command::cargo_bin("executor")
        .expect("Failed to find binary")
        .args([
            "execute",
            "--fname",
            "script.sh",
            "--path",
            "tests/test_data",
        ])
        .assert();

    assert.success();
}
