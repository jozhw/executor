use std::path::PathBuf;

use tempfile::TempDir;

#[derive(Debug)]
pub struct TemporaryPath {
    pub temp_dir: TempDir,
    pub temp_dir_path: PathBuf,
    pub temp_dir_full_path: PathBuf,
}
