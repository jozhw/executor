use std::{fs, path::PathBuf};
use tempfile::{tempdir, TempDir};
// Helper function to create a two nested directory structure
pub fn create_nested_directory_structure() -> Result<TempDir, std::io::Error> {
    let temp_dir: TempDir = tempdir()?;
    let nested_path: PathBuf = temp_dir.path().join("subdir1/subdir2");
    fs::create_dir_all(&nested_path)?;

    // Create a test file in the nested structure
    let file_path: PathBuf = nested_path.join("test_file.txt");
    fs::write(&file_path, "Test content")?;

    Ok(temp_dir)
}

// tests
#[cfg(test)]
mod tests {
    use std::fs;

    // Import the helper function you want to test
    use crate::utils::create_nested_directory_structure::create_nested_directory_structure;

    #[test]
    fn test_create_nested_directory_structure() {
        // Call the helper function to create the nested structure
        let temp_dir =
            create_nested_directory_structure().expect("Failed to create nested structure");

        // Check if the nested structure was created successfully
        let nested_path = temp_dir.path().join("subdir1/subdir2");
        assert!(nested_path.is_dir(), "Nested structure was not created");

        // Check if the test file was created
        let file_path = nested_path.join("test_file.txt");
        assert!(file_path.is_file(), "Test file was not created");

        // Check the content of the test file
        let content = fs::read_to_string(&file_path).expect("Failed to read test file");
        assert_eq!(
            content.trim(),
            "Test content",
            "Unexpected content in the test file"
        );
    }
}
