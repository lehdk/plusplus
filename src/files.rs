use std::{fs, path::PathBuf};
use dirs::home_dir;

pub use crate::cli;

pub fn init() {
    let result = fs::create_dir_all(dir());
    result.expect("Error creating directory");

    let _ = read_config();
}

pub fn dir() -> PathBuf {
    let home = home_dir().expect("Could not find home directory");

    home.join(".plusplus")
}

pub fn read_config() -> cli::Config {
    match fs::read_to_string(dir().join("plusplus.config")) {
        Ok(result) =>  {
            serde_json::from_str(&result).unwrap()
        }
        Err(_) => {
            let config = cli::Config {..Default::default()};
            write_config(&config);

            config
        },
    }
}

pub fn write_config(config: &cli::Config) {
    let json = serde_json::to_string(&config).unwrap();

    let _ = fs::write(dir().join("plusplus.config"), json);
}