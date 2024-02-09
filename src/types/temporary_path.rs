use std::path::PathBuf;

#[derive(Debug)]
pub struct TemporaryPath {
    pub temp_dir_path: PathBuf,
    pub temp_dir_full_path: PathBuf,
}
