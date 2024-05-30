use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "dots")]
#[command(about = "Dotfile Organization and Tracking System")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// initialize the dot files from a git repo
    Init {
        repo_url: String,
    },
    Link,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    repo_url: String,
    dotfiles: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { repo_url } => {
            println!("init {:?}", repo_url);
            return Ok(());
        }
        Commands::Link => {
            println!("link");
            return Ok(());
        }
    }
}
