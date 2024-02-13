use std::env;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// use the EntityType enum to organize subcommands that are to be called
#[derive(Debug, Subcommand)]
pub enum EntityType {
    Execute(ExecuteCommand),
    Delete(DeleteCommand),
    Search(SearchCommand),
}

impl EntityType {
    /// get the name of the file of interest for the execute and delete commands
    ///
    /// # Returns
    ///
    /// A string reference (&str) to the argument name input.
    pub fn get_name(&self) -> &str {
        match self {
            Self::Execute(args) => &args.fname,
            Self::Delete(args) => &args.fname,
            Self::Search(args) => &args.regex,
        }
    }

    /// get the path of the starting directory for the execute and delete commands
    ///
    /// # Notes
    ///
    /// If a directory path is not provided, it will use the current directory
    ///
    /// # Returns
    ///
    /// A PathBuf type of the path of the directory where execution, searching, deletion will occur
    pub fn get_path(&self) -> PathBuf {
        // get the current working directory and use in place if directory is not inputed
        let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");

        match self {
            Self::Execute(args) => args.path.clone().unwrap_or(current_dir),
            Self::Delete(args) => args.path.clone().unwrap_or(current_dir),
            Self::Search(args) => args.path.clone().unwrap_or(current_dir),
        }
    }
    /// Get the depth of the search
    ///
    /// # Returns
    ///
    /// An OPTION object.
    pub fn get_depth(&self) -> &Option<i32> {
        match self {
            Self::Search(args) => &args.depth,
            Self::Execute(args) => &args.depth,
            Self::Delete(args) => &args.depth,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Args, Clone)]
pub struct ExecuteCommand {
    /// Name of the file to execute.
    #[arg(short, long)]
    pub fname: String,

    /// Optional: Path to the root directory to begin the file execution search.
    /// root directory to initiate execution search
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Optional: Give the depth of the execution search
    /// default will search through all nested directories of the root
    #[arg(short, long)]
    pub depth: Option<i32>,
}

#[derive(Debug, Args, Clone)]
pub struct DeleteCommand {
    /// Name of the file to execute.
    #[arg(short, long)]
    pub fname: String,

    /// Optional: Path to the root directory to begin the file deletion search.
    /// root directory to initiate deletion search
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Optional: Give the depth of the deletion search
    /// default will search through all nested directories of the root
    #[arg(short, long)]
    pub depth: Option<i32>,
}

#[derive(Debug, Args, Clone)]
pub struct SearchCommand {
    /// Name of the file to execute (supports regex).
    #[arg(short, long)]
    pub regex: String,

    /// Optional: Path to the root directory to begin the search.
    /// root directory to initiate deletion search
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Optional: Give the depth of the search
    /// default will search through all nested directories of the root
    #[arg(short, long)]
    pub depth: Option<i32>,
}

// tests
//
// Note: only one command needs to be tested because the methods work the same
// regardless of the commands used

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Cli;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }

    #[test]
    fn test_get_name() {
        let args: ExecuteCommand = ExecuteCommand {
            fname: "script.sh".to_string(),
            path: Some(PathBuf::from("tests/test_data")),
            depth: None,
        };

        let entity_type: EntityType = EntityType::Execute(args.clone());

        assert_eq!(entity_type.get_name(), &args.fname);
    }

    #[test]
    fn test_get_path() {
        let args: ExecuteCommand = ExecuteCommand {
            fname: "script.sh".to_string(),
            path: Some(PathBuf::from("tests/test_data")),
            depth: None,
        };

        let entity_type: EntityType = EntityType::Execute(args.clone());

        assert_eq!(entity_type.get_path(), args.path.unwrap());
    }

    #[test]
    fn test_get_depth() {
        let args: ExecuteCommand = ExecuteCommand {
            fname: "script.sh".to_string(),
            path: Some(PathBuf::from("tests/test_data")),
            depth: Some(1),
        };

        let entity_type: EntityType = EntityType::Execute(args.clone());

        assert_eq!(entity_type.get_depth(), &args.depth);
    }
}
