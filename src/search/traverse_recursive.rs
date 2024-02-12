use regex::Regex;
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use crate::{
    errors::search_error::SearchError,
    types::{search_match::SearchMatch, traverse_match::TraverseMatch},
};

/// recursive algorithm to search through the file system and mutates the wrapper
/// function's TraverseMatch object
///
///
/// # Arguments
///
/// * 'dir' - reference to a PathBuf
/// * 'regex' - valid regex
/// * 'depth' - reference to an optional i32
/// * 'counter' - i32 that is used to count the depth
/// * 'acc' - accumulate the TraverseMatch object for tail recursion
///
/// # Returns
///
///
pub fn traverse_recursive(
    dir: &PathBuf,
    regex: &Regex,
    depth: &Option<i32>,
    counter: i32,
    acc: &mut TraverseMatch,
) -> Result<(), SearchError> {
    let entries: fs::ReadDir = fs::read_dir(dir).map_err(SearchError::IoError)?;

    for entry in entries {
        {
            let entry: DirEntry = entry.map_err(SearchError::IoError)?;
            let path: PathBuf = entry.path();

            if path.is_dir() {
                match depth {
                    Some(depth_value) if depth_value == &(counter + 1) => {}
                    _ => {
                        traverse_recursive(&path, regex, depth, counter + 1, acc)?;
                    }
                }
            } else if path.is_file() {
                // if file name exists
                if let Some(file_name) = path.file_name() {
                    // match the pattern
                    if regex.is_match(file_name.to_str().unwrap()) {
                        acc.match_count += 1;
                        let os_fname: Option<&std::ffi::OsStr> = path.file_name();
                        // convert OsStr into String type
                        match os_fname {
                            None => {}
                            Some(os_fname) => {
                                let fname: String = os_fname.to_string_lossy().to_string();
                                acc.files_matched.push(SearchMatch { fname, path });
                            }
                        }
                    }
                }
            }
        } // entry is dropped here
    }

    Ok(())
}
