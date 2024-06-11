use clap::{ Parser, Subcommand, Args};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub github_token: String,
    pub user: String
}

impl Default for Config {
    fn default() -> Config {
        Config {
            github_token: String::from(""),
            user: String::from("")
        }
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Root {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    GitToken(GitTokenCommands),
    #[command(subcommand)]
    GitRepositories(GitRepositoriesCommands)
}
    
#[derive(Debug, Subcommand)]
pub enum GitTokenCommands {
    Get,
    Set(SetGithubToken)
}

#[derive(Debug, Subcommand)]
pub enum GitRepositoriesCommands {
    List,
}

#[derive(Debug, Args)]
pub struct SetGithubToken {
    pub token: String
}