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
