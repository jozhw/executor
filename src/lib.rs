// public modules
pub mod cli;

// public structs
pub use cli::Cli;

// delete methods
pub mod delete_command {
    pub mod delete;
    pub mod delete_file;
    pub mod traverse_and_delete;
}

// execute methods
pub mod execute_command {
    pub mod execute;
    pub mod execute_file;
    pub mod traverse_and_execute;
}

// search methods
pub mod search_command {
    pub mod search;
    pub mod search_files;
    pub mod traverse_and_search;
    pub mod traverse_recursive;
}

// custom errors
pub mod errors {
    pub mod deletion_error;
    pub mod execution_error;
    pub mod search_error;
}

// types
pub mod types {
    pub mod search_match;
    pub mod traverse_match;
    pub mod traverse_result;
}

// utils
mod utils {
    pub mod create_nested_directory_structure;
}
