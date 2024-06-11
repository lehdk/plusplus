use clap::Parser;
use cli::{Commands, Root};

pub mod files;
pub mod cli;
pub mod github;

fn gitrepositories(value: cli::GitRepositoriesCommands) {

    match value {
        cli::GitRepositoriesCommands::List => {
            match github::get_repositories() {
                Err(err) => println!("{}", err),
                Ok(value) => {
                    for (i, el) in value.iter().enumerate() {
                        println!("{} - {}", i, el.name);
                    }
                }
            };
        },
    };
}

fn gittoken(value: cli::GitTokenCommands) {
    let mut config = files::read_config();

    match value {
        cli::GitTokenCommands::Get => println!("Token: {}", config.github_token),
        cli::GitTokenCommands::Set(token) => {
            config.github_token = token.token;
            files::write_config(&config);
        }
    };
}

fn main() {
    let _ = files::init();

    let opts = Root::parse();

    match opts.command {
        Commands::GitToken(value) => gittoken(value),
        Commands::GitRepositories(value) => gitrepositories(value)
    }
}
