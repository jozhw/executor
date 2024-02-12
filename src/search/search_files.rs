use crate::{errors::search_error::SearchError, types::traverse_match::TraverseMatch};
use std::path::PathBuf;

use regex::Regex;

use super::traverse_and_search::traverse_search;

/// Execute the traverse search given a regex pattern.
///
/// # Arguments
///
/// * 'dir' - reference to a PathBuf
/// * 'pattern' - regex pattern from input
/// * 'depth' - reference to an Optional i32 with integer input of the degree of traversal
///
/// # Returns
///
/// Result<TraverseMatch, SearchError> type, which must be handled by the main code
pub fn search_files(
    dir: &PathBuf,
    pattern: &str,
    depth: &Option<i32>,
) -> Result<TraverseMatch, SearchError> {
    let counter: i32 = 0;

    let regex: Regex =
        Regex::new(pattern).map_err(|e| SearchError::InvalidRegexPattern(e.to_string()))?;

    traverse_search(dir, &regex, depth, counter)
}

// no need for testing for the search_files method as that would be redundant since it is a wrapper
// function
