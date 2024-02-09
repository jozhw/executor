use std::{fs::File, io::Write, path::PathBuf};

use tempfile::{tempdir, TempDir};

use crate::types::temporary_path::TemporaryPath;

pub fn create_temp_directory_with_script(script_content: &str, depth: i32) -> TemporaryPath {
    let temp_dir: TempDir = tempdir().expect("Failed to create temporary directory");
    let temp_dir_path: PathBuf = temp_dir.path().to_path_buf();

    // Create nested directories
    let mut temp_dir_full_path: PathBuf = temp_dir.path().to_path_buf();
    for _ in 0..depth {
        let nested_dir = tempdir()
            .expect("Failed to create nested directory")
            .into_path();
        temp_dir_full_path.push(nested_dir);
    }

    // Create the script file with the provided content
    let script_path: PathBuf = temp_dir_full_path.join("script.sh");
    let mut script_file: File = File::create(&script_path).expect("Failed to create script file");
    write!(script_file, "{}", script_content).expect("Failed to write script file");

    TemporaryPath {
        temp_dir,
        temp_dir_path,
        temp_dir_full_path,
    }
}

#[cfg(test)]
mod tests {
    use crate::types::temporary_path::TemporaryPath;

    use super::create_temp_directory_with_script;
    use std::{fs, path::PathBuf};

    #[test]
    fn test_create_temp_directory_with_script() {
        let script_content: &str = "echo 'Hello world!'";
        let depth: i32 = 2;
        let temp_path: TemporaryPath = create_temp_directory_with_script(script_content, depth);
        let temp_dir_full_path: PathBuf = temp_path.temp_dir_full_path;

        // Check if the temporary directory and script file were created
        assert!(
            temp_dir_full_path.is_dir(),
            "Temporary directory was not created"
        );
        let script_path = temp_dir_full_path.join("script.sh");
        assert!(
            script_path.is_file(),
            "Script file was not created at {}",
            script_path.display()
        );

        // Check script file content matches with input
        let actual_content = fs::read_to_string(&script_path).expect("Failed to read script file");
        assert_eq!(actual_content.trim(), script_content.trim());
    }
}
