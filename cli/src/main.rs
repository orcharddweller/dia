use clap::{Parser, Subcommand};
use dia::build::build;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Build {},
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Build {}) => build(),
        None => {
            println!("No command provided");
        }
    }
}
