use assert_cmd::Command;
use std::fs::File;
use std::path::PathBuf;
use tempfile::{tempdir, TempDir};

#[test]
fn test_search_command_without_path() {
    // create a temporary directory for testing
    let temp_dir: TempDir = tempdir().expect("Failed to create temporary directory");
    let temp_dir_path: PathBuf = temp_dir.path().to_path_buf();

    // create files with different names in the temporary directory
    let file_names = ["file1.txt", "file2.txt", "file3.doc"];
    for &file_name in &file_names {
        let file_path = temp_dir_path.join(file_name);
        File::create(file_path).expect("Failed to create test file");
    }

    let assert = Command::cargo_bin("executor")
        .expect("Failed to find binary")
        .current_dir(temp_dir.path())
        .args(["search", "--regex", r"file\d\", "--depth", "1"])
        .assert();

    assert.success();

    // delete temp dir
    temp_dir
        .close()
        .expect("Failed to delete temporary directory");
}

// with --path and nested
#[test]
fn test_search_command_nested_without_depth() {
    let assert = Command::cargo_bin("executor")
        .expect("Failed to find binary")
        .args([
            "search",
            "--regex",
            "script.sh",
            "--path",
            "tests/test_data",
        ])
        .assert();

    assert.success();
}
