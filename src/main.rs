use std::path::PathBuf;

use clap::Parser;

use executor::delete_command::delete::delete;
use executor::execute_command::execute::execute;
use executor::search_command::search::search;
use executor::{cli::EntityType, Cli};

fn main() {
    let cli: Cli = Cli::parse();
    match &cli.entity_type {
        EntityType::Execute(args) => {
            // prep variables for method
            let entity_type: EntityType = EntityType::Execute(args.clone());
            let fname: &str = entity_type.get_name();
            let path: PathBuf = entity_type.get_path();
            let depth: &Option<i32> = entity_type.get_depth();

            match execute(&path, fname, depth) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("{:?}", err)
                }
            }
        }
        EntityType::Delete(args) => {
            // prep variables for method
            let entity_type: EntityType = EntityType::Delete(args.clone());
            let fname: &str = entity_type.get_name();
            let path: PathBuf = entity_type.get_path();
            let depth: &Option<i32> = entity_type.get_depth();

            match delete(&path, fname, depth) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("{:?}", err)
                }
            }
        }
        EntityType::Search(args) => {
            // prep variables for method
            let entity_type: EntityType = EntityType::Search(args.clone());
            let pattern: &str = entity_type.get_name();
            let path: PathBuf = entity_type.get_path();
            let depth: &Option<i32> = entity_type.get_depth();

            match search(&path, pattern, depth) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("{:?}", err)
                }
            }
        }
    }
}
