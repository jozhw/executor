use regex::Regex;
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use crate::{
    errors::search_error::SearchError,
    types::{search_match::SearchMatch, traverse_match::TraverseMatch},
};

pub fn traverse_search(
    dir: &PathBuf,
    regex: &Regex,
    depth: &Option<i32>,
    counter: i32,
) -> Result<TraverseMatch, SearchError> {
    // set up traverse return variables
    let mut match_count: i32 = 0;
    let mut files_matched: Vec<SearchMatch> = Vec::new();

    // get all contents of current directory
    let entries: fs::ReadDir = fs::read_dir(dir).map_err(SearchError::IoError)?;

    for entry in entries {
        let entry: DirEntry = entry.map_err(SearchError::IoError)?;
        let path: PathBuf = entry.path();

        // if the entry is a directory then recursion given set conditions
        if path.is_dir() {
            match depth {
                Some(depth_value) if depth_value == &counter => {
                    // recursion breaks since depth has been reached
                }
                _ => {
                    let result = traverse_search(dir, regex, depth, counter)?;
                    match_count += result.match_count;
                    files_matched.extend(result.files_matched);
                }
            }
        } else if path.is_file() {
            // if file name exists
            if let Some(file_name) = path.file_name() {
                // match the pattern
                if regex.is_match(file_name.to_str().unwrap()) {
                    match_count += 1;
                    let os_fname: Option<&std::ffi::OsStr> = path.file_name();
                    // conver OsStr into String type
                    match os_fname {
                        None => {}
                        Some(os_fname) => {
                            let fname: String = os_fname.to_string_lossy().to_string();
                            files_matched.push(SearchMatch { fname, path });
                        }
                    }
                }
            }
        }
    }
    Ok(TraverseMatch {
        match_count,
        files_matched,
    })
}
