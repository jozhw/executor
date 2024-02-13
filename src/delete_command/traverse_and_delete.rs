use std::error::Error;
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::PathBuf;

use crate::errors::deletion_error::DeletionError;
use crate::types::traverse_result::TraverseResult;

use super::delete_file::delete_file;

pub fn traverse_and_delete(
    dir: &PathBuf,
    fname: &str,
    depth: &Option<i32>,
    counter: i32,
) -> Result<TraverseResult, DeletionError> {
    // init traverse result return variables
    let mut successful_commands: i32 = 0;
    let mut unsuccessful_commands: i32 = 0;
    let mut all_paths: Vec<PathBuf> = Vec::new();
    let mut successful_paths: Vec<PathBuf> = Vec::new();
    let mut unsuccessful_paths: Vec<PathBuf> = Vec::new();

    // get all files within the current directory
    let entries: ReadDir = fs::read_dir(dir).map_err(DeletionError::IoError)?;

    for entry in entries {
        let entry: DirEntry = entry.map_err(DeletionError::IoError)?;
        let path: PathBuf = entry.path();
        if path.is_dir() {
            // determine whether or not to
            // recursively traverse subdirectories
            match depth {
                Some(depth_value) if depth_value == &counter => {
                    // check to see if the depth value has been reached
                    // recursion breaks
                }
                // recurse through until no longer directory
                _ => {
                    let result = traverse_and_delete(&path, fname, depth, counter + 1)?;
                    successful_commands += result.successful_commands;
                    unsuccessful_commands += result.unsuccessful_commands;
                    all_paths.extend(result.all_paths);
                    successful_paths.extend(result.successful_paths);
                    unsuccessful_paths.extend(result.unsuccessful_paths);
                }
            }
        } else if path.file_name().map_or(false, |name| name == fname) {
            // check if the file exists before attempting to delete
            if path.is_file() {
                // delete the file with the target name
                match delete_file(path.to_str().unwrap()) {
                    Ok(_) => {
                        println!("Deletion of {:?} successful.", path);
                        successful_commands += 1;
                        successful_paths.push(path.clone());
                    }
                    Err(err) => {
                        eprintln!("Error deleting {:?}: {}", path, err);
                        if let Some(source) = err.source() {
                            eprintln!("Caused by: {}", source);
                        }
                        unsuccessful_commands += 1;
                        unsuccessful_paths.push(path.clone());
                    }
                }
            } else {
                // The entry is not a file, so it's not eligible for deletion
                unsuccessful_commands += 1;
                unsuccessful_paths.push(path.clone());
            }

            // track all paths regardless of success or failure
            all_paths.push(path.clone());
        }
    }

    Ok(TraverseResult {
        successful_commands,
        unsuccessful_commands,
        all_paths,
        successful_paths,
        unsuccessful_paths,
    })
}

// tests
//

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use tempfile::TempDir;

    use super::traverse_and_delete;
    use crate::{
        types::traverse_result::TraverseResult,
        utils::create_nested_directory_structure::create_nested_directory_structure,
    };

    #[test]
    fn test_traverse_and_delete_success() {
        // Create a temporary directory with nested structure
        let temp_dir: TempDir =
            create_nested_directory_structure().expect("Failed to create nested structure");

        let temp_dir_path: PathBuf = temp_dir.path().to_path_buf();
        // Call your function
        let result: Result<
            crate::types::traverse_result::TraverseResult,
            crate::errors::deletion_error::DeletionError,
        > = traverse_and_delete(&temp_dir_path, "test_file.txt", &None, 0);

        // Check if the deletion was successful
        assert!(result.is_ok(), "Deletion failed: {:?}", result);

        // Check the number of successful and unsuccessful commands
        let result: TraverseResult = result.unwrap();
        assert_eq!(
            result.successful_commands, 1,
            "Expected 1 successful command, got {}",
            result.successful_commands
        );
        assert_eq!(
            result.unsuccessful_commands, 0,
            "Expected 0 unsuccessful commands, got {}",
            result.unsuccessful_commands
        );

        // Check if the file was deleted
        let file_path: PathBuf = temp_dir.path().join("subdir1/subdir2/test_file.txt");
        assert!(!file_path.exists(), "File should not exist after deletion");
    }

    #[test]
    fn test_traverse_and_delete_success_at_depth() {
        // Create a temporary directory with nested structure
        let temp_dir: TempDir =
            create_nested_directory_structure().expect("Failed to create nested structure");

        let temp_dir_path: PathBuf = temp_dir.path().to_path_buf();
        // Call your function
        let result: Result<
            crate::types::traverse_result::TraverseResult,
            crate::errors::deletion_error::DeletionError,
        > = traverse_and_delete(&temp_dir_path, "test_file.txt", &Some(1), 0);

        // Check if the deletion was successful
        assert!(result.is_ok(), "Deletion failed: {:?}", result);

        // Check the number of successful and unsuccessful commands
        let result: TraverseResult = result.unwrap();
        assert_eq!(
            result.successful_commands, 0,
            "Expected 0 successful command, got {}",
            result.successful_commands
        );
        assert_eq!(
            result.unsuccessful_commands, 0,
            "Expected 0 unsuccessful commands, got {}",
            result.unsuccessful_commands
        );

        // Check if the file was deleted
        let file_path: PathBuf = temp_dir.path().join("subdir1/subdir2/test_file.txt");
        assert!(file_path.exists(), "File should exist after deletion");
    }
}
