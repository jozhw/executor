use std::{error::Error, path::PathBuf};

use super::search_files::search_files;

/// serves as a wrapper function for search_files method and also handles the errors
/// this is the function that will be used in main.rs
///
/// # Arguments
///
/// * 'dir' - reference to a PathBuf
/// * 'pattern' - regex pattern to find file matches
/// * 'depth' - reference to an optional integer (i32)
///
/// # Returns
///

pub fn search(dir: &PathBuf, pattern: &str, depth: &Option<i32>) -> Result<(), Box<dyn Error>> {
    let result: Result<
        crate::types::traverse_match::TraverseMatch,
        crate::errors::search_error::SearchError,
    > = search_files(dir, pattern, depth);

    match result {
        Ok(res) => {
            // print out the number of successful matches
            println!("Number of successful matches: {:?}", res.match_count);
            // print out successful paths
            println!("Successful files matched: {:?}", res.files_matched);
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
