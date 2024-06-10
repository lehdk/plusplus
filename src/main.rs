use clap::
{ Parser, Subcommand, Args};
use std::{fs, path::PathBuf};
use serde::{Serialize, Deserialize};

use dirs::home_dir;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    github_token: String
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Root {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(subcommand)]
    GitToken(GitTokenCommands)
}
    
#[derive(Debug, Subcommand)]
enum GitTokenCommands {
    Get,
    Set(SetGithubToken)
}

#[derive(Debug, Args)]
pub struct SetGithubToken {
    pub token: String
}

fn gittoken(value: GitTokenCommands) {
    let mut config = read_config();

    match value {
        GitTokenCommands::Get => print!("Token: {}\n", config.github_token),
        GitTokenCommands::Set(token) => {
            config.github_token = token.token;
            write_config(&config);
        }
    }
}

fn dir() -> PathBuf {
    let home = home_dir().expect("Could not find home directory");

    home.join(".plusplus")
}

fn init() {
    let result = fs::create_dir_all(dir());
    result.expect("Error creating directory");

    let _ = read_config();
}

fn read_config() -> Config {
    match fs::read_to_string(dir().join("plusplus.config")) {
        Ok(result) =>  {
            serde_json::from_str(&result).unwrap()
        }
        Err(_) => {
            let config = Config { github_token: String::from("")};
            write_config(&config);

            config
        },
    }
}

fn write_config(config: &Config) {
    let json = serde_json::to_string(&config).unwrap();

    let _ = fs::write(dir().join("plusplus.config"), json);
}

fn main() {
    
    init();

    let opts = Root::parse();

    match opts.command {
        Commands::GitToken(value) => gittoken(value)
    }
}
