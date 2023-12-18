use std::env;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// use the EntityType enum to organize subcommands that are to be called
#[derive(Debug, Subcommand)]
pub enum EntityType {
    Execute(ExecuteCommand),
    Delete(DeleteCommand),
}

impl EntityType {
    /// get the name of the file of interest for the execute and delete commands
    pub fn get_name(&self) -> &str {
        match self {
            Self::Execute(args) => &args.fname,
            Self::Delete(args) => &args.fname,
        }
    }

    /// get the path of the starting directory for the execute and delete commands
    pub fn get_path(&self) -> PathBuf {
        // get the current working directory and use in place if directory is not inputed
        let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");

        match self {
            Self::Execute(args) => args.path.clone().unwrap_or(current_dir),
            Self::Delete(args) => args.path.clone().unwrap_or(current_dir),
        }
    }
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Args, Clone)]
pub struct ExecuteCommand {
    #[arg(short, long)]
    pub fname: String,

    #[arg(short, long)]
    pub path: Option<PathBuf>,

    #[arg(short, long)]
    pub depth: i32,
}

#[derive(Debug, Args, Clone)]
pub struct DeleteCommand {
    #[arg(short, long)]
    pub fname: String,

    #[arg(short, long)]
    pub path: Option<PathBuf>,

    #[arg(short, long)]
    pub depth: i32,
}
