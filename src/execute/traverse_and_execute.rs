use std::error::Error;
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::PathBuf;

use crate::errors::execution_error::ExecutionError;
use crate::types::traverse_result::TraverseResult;

use super::execute_file::execute_file;

pub fn traverse_and_execute(
    dir: &PathBuf,
    fname: &str,
    depth: &Option<i32>,
    counter: i32,
) -> Result<TraverseResult, ExecutionError> {
    // init traverse result return variables
    let mut successful_commands: i32 = 0;
    let mut unsuccessful_commands: i32 = 0;
    let mut all_paths: Vec<PathBuf> = Vec::new();
    let mut successful_paths: Vec<PathBuf> = Vec::new();
    let mut unsuccessful_paths: Vec<PathBuf> = Vec::new();

    // get all files within current directory
    let entries: ReadDir = fs::read_dir(dir).map_err(ExecutionError::IoError)?;

    for entry in entries {
        let entry: DirEntry = entry.map_err(ExecutionError::IoError)?;
        let path: PathBuf = entry.path();
        if path.is_dir() {
            // determine whether or not to
            // recursively traverse subdirectories
            match depth {
                Some(depth_value) if depth_value >= &counter => {
                    // check to see if depth value has been reached
                    // recursion breaks
                }
                // recurse through until no longer directory
                _ => {
                    let result = traverse_and_execute(&path, fname, depth, counter + 1)?;
                    successful_commands += result.successful_commands;
                    unsuccessful_commands += result.unsuccessful_commands;
                    all_paths.extend(result.all_paths);
                    successful_paths.extend(result.successful_paths);
                    unsuccessful_paths.extend(result.unsuccessful_paths);
                }
            }
        }
        // check if the file name matches the target file name
        else if path.file_name().map_or(false, |name| name == fname) {
            // execute the file with the target name
            match execute_file(path.to_str().unwrap()) {
                Ok(exit_status) => {
                    if exit_status.success() {
                        println!("Execution of {:?} successful.", path);
                        successful_commands += 1;
                        successful_paths.push(path.clone());
                    } else {
                        println!("Execution of {:?} unsuccessful.", path);
                        unsuccessful_commands += 1;
                        unsuccessful_paths.push(path.clone());
                    }
                }
                Err(err) => {
                    eprintln!("Error executing {:?}: {}", path, err);
                    if let Some(source) = err.source() {
                        eprintln!("Caused by: {}", source);
                    }
                    unsuccessful_commands += 1;
                    unsuccessful_paths.push(path.clone());
                }
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
    use crate::types::traverse_result::TraverseResult;

    use super::traverse_and_execute;

    use std::env;
    use std::path::PathBuf;
    use std::process::Command;

    #[test]
    fn test_traverse_and_execute_success() {
        // set the test directory to root/tests/test_data
        let mut current_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        current_dir.push("tests");
        current_dir.push("test_data");
        env::set_current_dir(&current_dir)
            .expect("Failed to set current directory to tests/test_data");

        // init logger to print out debug messages
        env_logger::init();

        // prepare arguments for traverse_and_execute function
        let fname: &str = "script.sh";
        let depth: Option<i32> = None;
        let counter: i32 = 0;
        let result: Result<TraverseResult, crate::errors::execution_error::ExecutionError> =
            traverse_and_execute(&current_dir, fname, &depth, counter);

        // assert that traverse_and_execute executed successfully
        assert!(result.is_ok(), "Execution failed: {:?}", result);

        let successful_commands: i32 = result.as_ref().unwrap().successful_commands;

        assert_eq!(
            successful_commands, 10,
            "Expected 10 successful commands, got {}",
            successful_commands
        );

        // check output to see if the script executed successfully
        let script_output: std::process::Output = Command::new("sh")
            .arg(fname)
            .output()
            .expect("Failed to execute script");

        // convert output bytes to string
        let output_str: &str =
            std::str::from_utf8(&script_output.stdout).expect("Failed to convert output to string");

        // print the path of each that executed
        println!("Script Path: {}", current_dir.join(fname).display());

        // assert that the expected output is present
        assert!(
            output_str.contains("Hello world!"),
            "Expected output does not match: {:?}",
            output_str
        );

        // reset the current directory to the original
        env::set_current_dir(env!("CARGO_MANIFEST_DIR"))
            .expect("Failed to reset current directory to original");
    }
}
