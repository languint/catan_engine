use clap::Parser;

use crate::cli::{Cli, Commands};

mod cli;
mod commands;

fn print_header() {
    let version = env!("CARGO_PKG_VERSION");
    println!("----- catan_engine 🌾 {version} -----");
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    print_header();

    let res = match cli.command {
        Commands::Start {} => commands::start_command::start_command().await,
    };

    if let Err(e) = res {
        cli::log_error("ERR", &e, 0, Some(cli::CLI_RED_HEADER));
    }

    Ok(())
}
