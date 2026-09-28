use clap::Parser;
use log::error;

mod cli;
mod commands;

fn main() {
    env_logger::init();

    let args = cli::Cli::parse();

    if let Err(e) = commands::handle_command(args.command) {
        error!("Error executing command: {}", e);
        std::process::exit(1);
    }
}
