mod cli;
mod commands;
mod error;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    env_logger::init();

    let args = Cli::parse();

    match args.command {
        Commands::Merge { input, output } => {
            if let Err(e) = commands::group_a::handle_merge(input, output) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Commands::Split { input, output_dir } => {
            if let Err(e) = commands::group_a::handle_split(input, output_dir) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Commands::Extract { input, pages, output } => {
            if let Err(e) = commands::group_a::handle_extract(input, pages, output) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Commands::Delete { input, pages, output } => {
            if let Err(e) = commands::group_a::handle_delete(input, pages, output) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Commands::Reorder { input, order, output } => {
            if let Err(e) = commands::group_a::handle_reorder(input, order, output) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Commands::Rotate { input, pages, degrees, output } => {
            if let Err(e) = commands::group_a::handle_rotate(input, pages, degrees, output) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Commands::Crop { input, pages, rect, output } => {
            if let Err(e) = commands::group_a::handle_crop(input, pages, rect, output) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Commands::Burst { input, output_dir } => {
            if let Err(e) = commands::group_a::handle_burst(input, output_dir) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
