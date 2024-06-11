use clap::Parser;
use cli::{Commands, Root};
use std::{fs, path::PathBuf};

use dirs::home_dir;

mod cli;

fn gittoken(value: cli::GitTokenCommands) {
    let mut config = read_config();

    match value {
        cli::GitTokenCommands::Get => print!("Token: {}\n", config.github_token),
        cli::GitTokenCommands::Set(token) => {
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

fn read_config() -> cli::Config {
    match fs::read_to_string(dir().join("plusplus.config")) {
        Ok(result) =>  {
            serde_json::from_str(&result).unwrap()
        }
        Err(_) => {
            let config = cli::Config { github_token: String::from("")};
            write_config(&config);

            config
        },
    }
}

fn write_config(config: &cli::Config) {
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
