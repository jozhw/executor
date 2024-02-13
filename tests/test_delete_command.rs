use assert_cmd::Command;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use tempfile::{tempdir, TempDir};

#[test]
fn test_delete_command_without_path() {
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
        .args(["delete", "--fname", "file1.txt", "--depth", "1"])
        .assert();

    assert.success();

    // delete temp dir
    temp_dir
        .close()
        .expect("Failed to delete temporary directory");
}

// with --path and nested
#[test]
fn test_delete_command_nested_without_depth() {
    // create the temp dir
    let temp_dir: TempDir = tempdir().expect("Failed to create temporary root directory");

    // create nested directories to attach to root temp dir
    let nested_path: PathBuf = temp_dir.path().join("subdir1/subdir2");
    fs::create_dir_all(&nested_path).unwrap();

    // create a test file in the nested structure
    let file_path: PathBuf = nested_path.join("test_file.txt");
    fs::write(&file_path, "Test content").unwrap();

    let assert = Command::cargo_bin("executor")
        .expect("Failed to find binary")
        .current_dir(temp_dir.path())
        .args(["delete", "--fname", "test_file.txt"])
        .assert();

    assert.success();

    // delete temp dir
    temp_dir
        .close()
        .expect("Failed to delete temporary directory");
}
