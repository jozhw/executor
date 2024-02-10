use std::{fs, path::PathBuf};
use tempfile::{tempdir, TempDir};
/// Helper function to create a two nested directory structure for traverse testing
///
/// # Arguments
///
/// None
///
/// # Returns
///
/// A TempDir in order to remain inscope for testing
pub fn create_nested_directory_structure() -> Result<TempDir, std::io::Error> {
    // create the temp dir
    let temp_dir: TempDir = tempdir().expect("Failed to create temporary root directory");

    // create nested directories to attach to root temp dir
    let nested_path: PathBuf = temp_dir.path().join("subdir1/subdir2");
    fs::create_dir_all(&nested_path)?;

    // create a test file in the nested structure
    let file_path: PathBuf = nested_path.join("test_file.txt");
    fs::write(&file_path, "Test content")?;

    Ok(temp_dir)
}

// tests
#[cfg(test)]
mod tests {

    use crate::utils::create_nested_directory_structure::create_nested_directory_structure;
    use std::fs;

    #[test]
    fn test_create_nested_directory_structure() {
        // Call the helper function to create the nested structure
        let temp_dir =
            create_nested_directory_structure().expect("Failed to create nested structure");

        // check if the nested structure was created successfully
        let nested_path = temp_dir.path().join("subdir1/subdir2");
        assert!(nested_path.is_dir(), "Nested structure was not created");

        // check if the test file was created
        let file_path = nested_path.join("test_file.txt");
        assert!(file_path.is_file(), "Test file was not created");

        // check the content of the test file
        let content = fs::read_to_string(&file_path).expect("Failed to read test file");
        assert_eq!(
            content.trim(),
            "Test content",
            "Unexpected content in the test file"
        );
    }
}
