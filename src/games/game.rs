use std::path::PathBuf;

pub enum Platforms {
    Windows,
    Mac,
    Linux,
}

pub struct GameData {
    name: String,
    available_platforms: Vec<Platforms>,
}

pub trait Game {
    fn download_game(&self, download_location: PathBuf) -> Result<(), String>;
    fn install_game(&self, install_location: PathBuf) -> Result<(), String>;
    fn update_game(&self, installed_game_location: PathBuf) -> Result<(), String>;
    fn launch_game(&self) -> Result<(), String>;
}
