use std::path::PathBuf;

pub struct TraverseResult {
    pub successful_commands: i32,
    pub unsuccessful_commands: i32,
    pub all_paths: Vec<PathBuf>,
    pub successful_paths: Vec<PathBuf>,
    pub unsuccessful_paths: Vec<PathBuf>,
}
