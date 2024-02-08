use std::error::Error;
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::PathBuf;

use crate::errors::execution_error::ExecutionError;

use super::execute_file::execute_file;

pub fn traverse_and_execute(
    dir: &PathBuf,
    fname: &str,
    depth: &Option<i32>,
    counter: i32,
) -> Result<(), ExecutionError> {
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
                _ => traverse_and_execute(&path, fname, depth, counter + 1)?,
            }
        }
        // check if the file name matches the target file name
        else if path.file_name().map_or(false, |name| name == fname) {
            // execute the file with the target name
            match execute_file(path.to_str().unwrap()) {
                Ok(exit_status) => {
                    if exit_status.success() {
                        println!("Execution of {:?} successful.", path)
                    } else {
                    }
                }
                Err(err) => {
                    eprintln!("Error executing {:?}: {}", path, err);
                    if let Some(source) = err.source() {
                        eprintln!("Caused by: {}", source);
                    }
                }
            }
        }
    }
    Ok(())
}
