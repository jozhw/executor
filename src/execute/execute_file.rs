use std::process::{Command, ExitStatus};

use crate::errors::execution_error::ExecutionError;

/// Execute the file of interest within the current working directory that has been traversed.
///
/// # Arguments
///
/// * 'path' - Referenced str: The path to the executable file.
///
/// # Returns
///
/// ExitStatus object
pub fn execute_file(path: &str) -> Result<std::process::ExitStatus, ExecutionError> {
    // create a command to run the executable
    let mut command: Command = Command::new(path);

    // for future development: argument implementation for execution of script
    // need to make sure that args is an argument for execute_file and the type
    // casting is a vector of borrowed strings (Vec<&str>)
    //
    // for arg in args {
    //      command.arg(arg);
    // }

    // execute the command and return the result
    let status: ExitStatus = match command.status() {
        Ok(status) => status,
        Err(e) => return Err(ExecutionError::IoError(e)),
    };

    if status.success() {
        Ok(status)
    } else {
        Err(ExecutionError::CommandError(
            format!("Failed to execute {:?}", path),
            status,
        ))
    }
}

// tests
//

#[cfg(test)]
mod tests {
    use super::execute_file;
    use crate::errors::execution_error::ExecutionError;
    use std::fs::{File, Permissions};
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_execute_file_success() {
        // create a temporary directory
        let temp_dir: TempDir =
            tempfile::tempdir().expect("Failed to create a temporary directory.");

        // create a test script file that will succeed in the temp dir
        let script_content: &str = r#"#!/bin/bash
            echo 'Test script executed successfully'"#;
        let script_path: PathBuf = temp_dir.path().join("test_script.sh");
        let mut script_file: File =
            File::create(&script_path).expect("Failed to create script file.");
        write!(script_file, "{}", script_content).expect("Failed to write to script file.");

        // make the script file executable
        std::process::Command::new("chmod")
            .args(&["+x", script_path.to_str().unwrap()])
            .output()
            .expect("Failed to make script executable.");

        // execute the file and check the result
        let result: Result<std::process::ExitStatus, ExecutionError> =
            execute_file(script_path.to_str().unwrap());

        // assert that the execution was successful
        assert!(result.is_ok(), "Execution failed: {:?}", result);
    }

    #[test]
    fn test_execute_file_failure() {
        // create a temporary directory
        let temp_dir: TempDir = tempfile::tempdir().expect("Failed to create temporary directory.");

        // create a non-executable file in the temporary directory
        let file_path: PathBuf = temp_dir.path().join("non_executable_file.txt");
        File::create(&file_path).expect("Failed to create file.");

        // execute the non-executable file and check the result
        let result: Result<std::process::ExitStatus, ExecutionError> =
            execute_file(file_path.to_str().unwrap());

        // assser that it failed
        assert!(
            result.is_err(),
            "Execution was successful, but it should have failed."
        );
    }

    #[test]
    fn test_execute_file_permission_failure() {
        // create a temporary directory
        let temp_dir: TempDir =
            tempfile::tempdir().expect("Failed to create a temporary directory.");

        // create a test cript in the temp dir
        let script_content: &str = r#"#!/bin/sh
            echo "Test script executed successfully""#;
        let script_path: PathBuf = temp_dir.path().join("test_script.sh");
        let mut script_file: File =
            File::create(&script_path).expect("Failed to create script file");
        write!(script_file, "{}", script_content).expect("Failed to write to script file");

        // make the script file non-executable
        let permissions: Permissions = Permissions::from_mode(0o644); // set to creat and write permissions only

        std::fs::set_permissions(&script_path, permissions)
            .expect("Failed to set permissions to read and write only.");

        // execute the file and check
        let result: Result<std::process::ExitStatus, ExecutionError> =
            execute_file(script_path.to_str().unwrap());

        // assert that the execute failed due to permissions
        assert!(
            result.is_err(),
            "Execution succeeded, but it should have failed."
        );
    }
}
