use clap::{Parser, Subcommand};
use dirs::home_dir;
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

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
        Commands::Init { repo_url } => init_command(repo_url),
        Commands::Link => {
            println!("link");
            return Ok(());
        }
    }
}

fn init_command(repo_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = clone_repo(repo_url)?;
    let dotfiles = scan_dotfiles(&repo_path)?;
    // create a config
    let config = Config {
        repo_url: repo_url.to_string(),
        dotfiles,
    };

    save_config("~/.config/config.toml", &config)?;

    println!("Repository configuration saved");

    // save config
    //
    // return
    Ok(())
}
fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = path.as_ref().expand_home()?;
    let config_content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

fn save_config<P: AsRef<Path>>(path: P, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = path.as_ref().expand_home()?;
    let config_dir = config_path.parent().unwrap();

    fs::create_dir_all(config_dir)?;

    let config_content = toml::to_string(config)?;
    fs::write(config_path, config_content)?;
    Ok(())
}

fn clone_repo(repo_url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = home_dir().expect("unable to find home dir");
    let repo_path = home.join(".dotfiles_repo");

    if repo_path.exists() {
        // @TODO consider removing this logic. Maybe just warn user if its exists
        fs::remove_dir_all(&repo_path)?;
    }

    Repository::clone(repo_url, &repo_path)?;

    Ok(repo_path)
}

fn scan_dotfiles(repo_path: &PathBuf) -> Result<Vec<String>, std::io::Error> {
    let mut dotfiles = Vec::new();

    for entry in fs::read_dir(repo_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(filename) = path.file_name() {
                if filename
                    .to_str()
                    .map(|s| s.starts_with('.'))
                    .unwrap_or(false)
                {
                    dotfiles.push(filename.to_string_lossy().into_owned());
                }
            }
        }
    }
    Ok(dotfiles)
}

trait ExpandHome {
    fn expand_home(&self) -> Result<PathBuf, &'static str>;
}

impl ExpandHome for Path {
    fn expand_home(&self) -> Result<PathBuf, &'static str> {
        if !self.starts_with("~") {
            return Ok(self.to_path_buf());
        }

        let home = home_dir().ok_or("home dir not found")?;
        Ok(home.join(self.strip_prefix("~").unwrap()))
    }
}
