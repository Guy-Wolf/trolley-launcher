use std::path::PathBuf;

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
    fn download_game(&self, download_location: PathBuf, install_location: PathBuf) -> Result<(), String>;
    fn update_game(&self, download_location: PathBuf, install_location: PathBuf) -> Result<(), String>;
    fn login_to_game(&self, username: String, password: String) -> Result<(), String>;
    fn launch_game(&self) -> Result<(), String>;
}
