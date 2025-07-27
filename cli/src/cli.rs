use clap::{Parser, Subcommand};
use owo_colors::{AnsiColors, DynColors, OwoColorize};
#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Start {},
}

pub fn log_header(header: &str, message: &str, depth: u8, color: Option<DynColors>) {
    let header_str = match color {
        Some(c) => header.color(c).to_string(),
        None => header.to_string(),
    };
    println!("{}{} {}", " ".repeat(depth as usize), header_str, message);
}

pub fn log_error(header: &str, message: &str, depth: u8, color: Option<DynColors>) {
    let header_str = match color {
        Some(c) => header.color(c).to_string(),
        None => header.to_string(),
    };
    eprintln!("{}{} {}", " ".repeat(depth as usize), header_str, message);
}

pub const CLI_PURPLE_HEADER: DynColors = DynColors::Ansi(AnsiColors::Magenta);
pub const CLI_GREEN_HEADER: DynColors = DynColors::Ansi(AnsiColors::Green);
pub const CLI_RED_HEADER: DynColors = DynColors::Ansi(AnsiColors::Red);
pub const CLI_BLUE_HEADER: DynColors = DynColors::Ansi(AnsiColors::Blue);
pub const CLI_YELLOW_HEADER: DynColors = DynColors::Ansi(AnsiColors::Yellow);
