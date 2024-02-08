// public modules
pub mod cli;

// public structs
pub use cli::Cli;

// delete methods
mod delete {
    pub mod delete_file;
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
    pub mod execution_error;
}
