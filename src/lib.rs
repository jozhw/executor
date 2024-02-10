// public modules
pub mod cli;

// public structs
pub use cli::Cli;

// delete methods
mod delete {
    pub mod delete_file;
    pub mod traverse_and_delete;
}

// execute methods
mod execute {
    pub mod execute_file;
    pub mod traverse_and_execute;
}

// search methods
mod search {
    pub mod search_file;
}

// custom errors
mod errors {
    pub mod deletion_error;
    pub mod execution_error;
    pub mod search_error;
}

// types
mod types {
    pub mod traverse_result;
}

// utils
mod utils {
    pub mod create_nested_directory_structure;
}
