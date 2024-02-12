use regex::Regex;
use std::path::PathBuf;

use crate::{
    errors::search_error::SearchError, search::traverse_recursive::traverse_recursive,
    types::traverse_match::TraverseMatch,
};

// must implement a tail recursion to avoid stack overflow

/// wrapper function for the traverse_recursive tail recursive algorithm to search through
/// the file system at a specified depth and return the matches
///
/// # Arguments
///
/// * 'dir' - reference to a PathBuf
/// * 'regex' - valid regex
/// * 'depth' - reference to an optional i32
/// * 'counter' - i32 that is used to count the depth
///
/// # Returns
///
/// TraverseMatch result that contains the number of matches and a array containing SearchMatch
/// objects
pub fn traverse_search(
    dir: &PathBuf,
    regex: &Regex,
    depth: &Option<i32>,
    counter: i32,
) -> Result<TraverseMatch, SearchError> {
    // create a result variable that acts as the accumulator for the tail recursion
    let mut result = TraverseMatch {
        match_count: 0,
        files_matched: Vec::new(),
    };

    traverse_recursive(dir, regex, depth, counter, &mut result)?;

    Ok(result)
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::create_nested_directory_structure::create_nested_directory_structure;
    use std::fs::File;
    use tempfile::{tempdir, TempDir};

    #[test]
    fn test_traverse_search_success() {
        // create a temporary directory for testing
        let temp_dir: TempDir = tempdir().expect("Failed to create temporary directory");
        let temp_dir_path: PathBuf = temp_dir.path().to_path_buf();

        // create files with different names in the temporary directory
        let file_names = ["file1.txt", "file2.txt", "file3.doc"];
        for &file_name in &file_names {
            let file_path = temp_dir_path.join(file_name);
            File::create(file_path).expect("Failed to create test file");
        }

        // execute the traverse_search function with a regex pattern
        let pattern: &str = r"file\d\.txt";
        let depth: Option<i32> = None;
        let regex: Regex = Regex::new(pattern).expect("Failed to create regex");
        let result: Result<TraverseMatch, SearchError> =
            traverse_search(&temp_dir_path, &regex, &depth, 0);

        // assert that the traverse_search was successful
        assert!(result.is_ok(), "Traversal failed: {:?}", result);

        // check the content of the TraverseMatch
        let traverse_match: TraverseMatch = result.unwrap();
        assert_eq!(traverse_match.match_count, 2, "Expected 2 matches");

        // delete tempdir
        temp_dir
            .close()
            .expect("Failed to remove temporary directory");
    }

    #[test]
    fn test_traverse_search_nested() {
        let temp_dir: TempDir =
            create_nested_directory_structure().expect("Failed to create nested structure");
        let temp_dir_path: PathBuf = temp_dir.path().to_path_buf();

        // execute the traverse_search function with a regex pattern
        let pattern: &str = r"test_file\.txt";
        let depth: Option<i32> = None;
        let regex: Regex = Regex::new(pattern).expect("Failed to create regex");
        let result: Result<TraverseMatch, SearchError> =
            traverse_search(&temp_dir_path, &regex, &depth, 0);

        // assert that the traverse_search was successful
        assert!(result.is_ok(), "Traversal failed: {:?}", result);

        // check the content of the TraverseMatch
        let traverse_match: TraverseMatch = result.unwrap();
        assert_eq!(traverse_match.match_count, 1, "Expected 1 match");

        // delete the temporary directory
        temp_dir
            .close()
            .expect("Failed to remove temporary directory");
    }
}
