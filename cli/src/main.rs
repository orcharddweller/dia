use std::path::Path;

use clap::{Parser, Subcommand};
use dia::{build::build, dev::dev, new::new};

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Build {},
    Dev {},
    New { name: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Build {}) => build(Path::new("dist")),
        Some(Commands::Dev {}) => dev(),
        Some(Commands::New { name }) => new(&name),
        None => {
            println!("No command provided");
        }
    }
}
