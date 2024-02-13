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
    use std::env;
    use std::path::PathBuf;

    #[cfg(not(windows))]
    #[test]
    fn test_execute_file_success() {
        // set the test directory to root/tests/test_data
        let mut current_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        current_dir.push("tests");
        current_dir.push("test_data");
        env::set_current_dir(&current_dir)
            .expect("Failed to set current directory to tests/test_data");

        let script_path: PathBuf = current_dir.join("script.sh");
        // execute the file and check the result
        let result: Result<std::process::ExitStatus, ExecutionError> =
            execute_file(script_path.to_str().unwrap());

        // assert that the execution was successful
        assert!(result.is_ok(), "Execution failed: {:?}", result);

        // reset the current directory to the original
        env::set_current_dir(env!("CARGO_MANIFEST_DIR"))
            .expect("Failed to reset current directory to original");
    }

    #[cfg(not(windows))]
    #[test]
    fn test_execute_file_failure() {
        // set the test directory to root/tests/test_data
        let mut current_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        current_dir.push("tests");
        current_dir.push("test_data");
        env::set_current_dir(&current_dir)
            .expect("Failed to set current directory to tests/test_data");

        // create a non-executable file in the temporary directory
        let file_path: PathBuf = current_dir.join("non_executable_file.txt");

        // execute the non-executable file and check the result
        let result: Result<std::process::ExitStatus, ExecutionError> =
            execute_file(file_path.to_str().unwrap());

        // assser that it failed
        assert!(
            result.is_err(),
            "Execution was successful, but it should have failed."
        );

        // reset the current directory to the original
        env::set_current_dir(env!("CARGO_MANIFEST_DIR"))
            .expect("Failed to reset current directory to original");
    }

    #[cfg(not(windows))]
    #[test]
    fn test_execute_file_permission_failure() {
        // set the test directory to root/tests/test_data
        let mut current_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        current_dir.push("tests");
        current_dir.push("test_data");
        env::set_current_dir(&current_dir)
            .expect("Failed to set current directory to tests/test_data");

        // create a non-executable file in the temporary directory
        let script_path: PathBuf = current_dir.join("failed_script.sh");

        // execute the file and check
        let result: Result<std::process::ExitStatus, ExecutionError> =
            execute_file(script_path.to_str().unwrap());

        // assert that the execute failed due to permissions
        assert!(
            result.is_err(),
            "Execution succeeded, but it should have failed."
        );

        // reset the current directory to the original
        env::set_current_dir(env!("CARGO_MANIFEST_DIR"))
            .expect("Failed to reset current directory to original");
    }
}
