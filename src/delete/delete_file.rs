use crate::errors::deletion_error::DeletionError;
use std::fs;

/// Delete the file of interest within the current working directory that has been traversed.
///
/// # Arguments
///
/// * 'path' - Referenced str: The path to the executable file.
///
/// # Returns
///
/// ExitStatus object
pub fn delete_file(path: &str) -> Result<(), DeletionError> {
    // create a command to run the executable

    // delete the command and return the result
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Err(DeletionError::NotFound)
            } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                Err(DeletionError::PermissionDenied)
            } else {
                Err(DeletionError::Other(format!("{}", e)))
            }
        }
    }
}

// tests
//

#[cfg(test)]
mod tests {
    use super::delete_file;
    use crate::errors::deletion_error::DeletionError;
    use crate::types::temporary_path::TemporaryPath;
    use crate::utils::create_temp_directory_with_script::create_temp_directory_with_script;
    use std::path::PathBuf;

    #[test]
    fn test_delete_file_success() {
        let depth: i32 = 1;
        let script_content: &str = "echo Hello World!";
        let temp_path: TemporaryPath = create_temp_directory_with_script(script_content, depth);
        let temp_dir_full_path: PathBuf = temp_path.temp_dir_full_path;
        let script_path: PathBuf = temp_dir_full_path.join("script.sh");

        // delete the file and check the result
        let result: Result<(), DeletionError> = delete_file(script_path.to_str().unwrap());

        // assert that the execution was successful
        assert!(result.is_ok(), "Deletion failed: {:?}", result);

        // check to see if file still exists
        assert!(
            temp_dir_full_path.join("script.sh").metadata().is_err(),
            "File should not exist after deletion"
        )
    }
}
