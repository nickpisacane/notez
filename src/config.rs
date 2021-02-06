use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs,
    io::{Error, ErrorKind, Result},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub debounce: u64,
    pub directories: Vec<String>,
}

impl Config {
    pub fn get_file() -> Result<String> {
        let home = match home_dir() {
            Some(home) => home,
            None => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "Failed to find config file",
                ))
            }
        };
        match home.join(".notez_config.json").to_str() {
            Some(file) => Ok(String::from(file)),
            None => Err(Error::new(
                ErrorKind::NotFound,
                "Failed to find config file",
            )),
        }
    }

    pub fn read() -> Result<Config> {
        let config_file = Config::get_file()?;
        let file_contents = fs::read_to_string(config_file)?;
        let config: Config = serde_json::from_str(&file_contents.to_string())?;
        Ok(config)
    }

    pub fn find(&self, path: &str) -> Option<String> {
        for dir in self.directories.iter() {
            if Path::new(path).starts_with(dir) {
                return Some(dir.to_string());
            }
        }
        None
    }
}
