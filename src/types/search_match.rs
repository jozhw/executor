use std::path::PathBuf;

pub struct SearchMatch {
    // name of the file matched
    pub fname: String,
    // path of the file matched
    pub path: PathBuf,
}
