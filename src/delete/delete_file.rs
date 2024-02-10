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

    // for future development: argument implementation for execution of script
    // need to make sure that args is an argument for delete_file and the type
    // casting is a vector of borrowed strings (Vec<&str>)
    //
    // for arg in args {
    //      command.arg(arg);
    // }

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
    use std::fs::File;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_delete_file_success() {
        // create a temporary directory
        let temp_dir: TempDir =
            tempfile::tempdir().expect("Failed to create a temporary directory.");

        // create a test script file that will be deleted in the temp dir
        let script_path: PathBuf = temp_dir.path().join("test_script.sh");
        File::create(&script_path).expect("Failed to create script file.");

        // delete the file and check the result
        let result: Result<(), DeletionError> = delete_file(script_path.to_str().unwrap());

        // assert that the execution was successful
        assert!(result.is_ok(), "Deletion failed: {:?}", result);
    }
}
