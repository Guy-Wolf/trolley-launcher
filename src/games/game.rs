use std::path::PathBuf;

use serde_json::Value;

pub enum Platforms {
    Windows,
    Mac,
    Linux,
}

pub struct GameData {
    pub name: String,
    pub available_platforms: Vec<Platforms>,
    pub is_installed: bool,
}

pub trait Game {
    fn download_game(&mut self, download_location: PathBuf, install_location: PathBuf) -> Result<(), String>;
    fn update_game(&self, download_location: PathBuf, install_location: PathBuf) -> Result<(), String>;
    fn launch_game(&self, username: String, password: String, install_location: PathBuf) -> Result<(), String>;
}
