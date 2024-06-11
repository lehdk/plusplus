use clap::Parser;
use cli::{Commands, Root};

mod cli;
mod github;
mod files;

fn gittoken(value: cli::GitTokenCommands) {
    let mut config = files::read_config();

    match value {
        cli::GitTokenCommands::Get => print!("Token: {}\n", config.github_token),
        cli::GitTokenCommands::Set(token) => {
            config.github_token = token.token;
            files::write_config(&config);
        }
    }
}
fn main() {
    files::init();

    let opts = Root::parse();

    match opts.command {
        Commands::GitToken(value) => gittoken(value)
    }
}
