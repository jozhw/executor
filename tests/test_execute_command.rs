use assert_cmd::Command;
use std::fs::{File, Permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_execute_command_without_path() {
    // create temporary dir to house all testing files
    let temp_dir: TempDir = tempfile::tempdir().expect("Failed to create temporary directory.");
    // create a test cript in the temp dir
    let script_content: &str = r#"#!/bin/sh
            echo "Test script executed successfully""#;
    let script_path: PathBuf = temp_dir.path().join("test_script.sh");
    let mut script_file: File = File::create(&script_path).expect("Failed to create script file");
    write!(script_file, "{}", script_content).expect("Failed to write to script file");

    // make the script file non-executable
    let permissions: Permissions = Permissions::from_mode(0o644); // set to creat and write permissions only

    std::fs::set_permissions(&script_path, permissions)
        .expect("Failed to set permissions to read and write only.");

    let assert = Command::cargo_bin("executor")
        .expect("Failed to find binary")
        .current_dir(temp_dir.path())
        .args(["execute", "--fname", "script.sh", "--depth", "1"])
        .assert();

    assert.success();

    // delete temp dir
    temp_dir
        .close()
        .expect("Failed to delete temporary directory");
}

// with --path and nested
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
