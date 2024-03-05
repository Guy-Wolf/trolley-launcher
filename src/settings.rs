use dirs::{data_dir, cache_dir, config_dir};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

const DEFAULT_CONFIG_FILENAME: &str = "config.toml";
const DOWNLOADS_DIR_NAME: &str = "trolley-downloads";
const INSTALL_DIR_NAME: &str = "trolley-games";

#[derive(Serialize, Deserialize)]
struct Config {
    download_path: PathBuf,
    install_path: PathBuf,
}

pub fn ensure_config_file() -> Result<(), String> {
    let config = Config {
        download_path: cache_dir()
            .ok_or("Could not find cache directory")?
            .join(DOWNLOADS_DIR_NAME),
        install_path: data_dir()
            .ok_or("Could not find data directory")?
            .join(INSTALL_DIR_NAME),
    };
    if !is_config_dir_present()? {
        create_config_dir()?;
    }
    let config_file = config_dir().unwrap().join(DEFAULT_CONFIG_FILENAME);
    let mut file = match File::create(&config_file) {
        Ok(file) => file,
        Err(_) => return Err("Could not create config file".to_string()),
    };
    match file.write_all(toml::to_string(&config).unwrap().as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Could not write to config file".to_string()),
    }
}

fn is_config_dir_present() -> Result<bool, String> {
    let config_file = config_dir()
        .ok_or("Could not find config directory")?
        .join(DEFAULT_CONFIG_FILENAME);
    Ok(config_file.exists())
}

fn create_config_dir() -> Result<(), String> {
    let config_dir = config_dir().ok_or("Could not find config directory")?;
    match create_dir_all(&config_dir) {
        Ok(_) => Ok(()),
        Err(_) => Err("Could not create config directory".to_string()),
    }
}
