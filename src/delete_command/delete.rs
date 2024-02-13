use std::{error::Error, path::PathBuf};

use super::traverse_and_delete::traverse_and_delete;

/// serves as a wrapper function for traverse_and_delete method and also handles the errors
/// this is the function that will be used in main.rs
///
/// # Arguments
///
/// * 'dir' - reference to a PathBuf
/// * 'fname' - the file name as a str reference
/// * 'depth' - reference to an optional integer (i32)
///
/// # Returns
///

pub fn delete(dir: &PathBuf, fname: &str, depth: &Option<i32>) -> Result<(), Box<dyn Error>> {
    // set counter for traverse_and_delete method
    let counter: i32 = 0;

    let result: Result<
        crate::types::traverse_result::TraverseResult,
        crate::errors::deletion_error::DeletionError,
    > = traverse_and_delete(dir, fname, depth, counter);

    match result {
        Ok(res) => {
            // print out the number of successful deletions
            println!(
                "Number of successful deletions: {:?}",
                res.successful_commands
            );
            // print out the number of failed deletions
            println!(
                "Number of failed deletions: {:?}",
                res.unsuccessful_commands
            );
            // print out successful paths
            println!("Successful files deleted: {:?}", res.successful_paths);
            // print out failed paths
            println!("Files failed to be deleted: {:?}", res.unsuccessful_paths);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            if let Some(source) = e.source() {
                eprintln!("Caused by: {}", source)
            }
        }
    }

    Ok(())
}
