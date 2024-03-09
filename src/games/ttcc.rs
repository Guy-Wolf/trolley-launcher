use super::game::{Game, GameData, Platforms};
use std::path::PathBuf;

const INSTALLER_PATH: &str =
    "https://github.com/CorporateClash/pyside2-releases/releases/download/v1.3.0/installer.exe";

pub struct TTCC {
    game_data: GameData,
    installer_path: PathBuf,
    args: Vec<String>,
}

impl TTCC {
    pub fn new(installer_path: PathBuf, args: Vec<String>) -> Self {
        Self {
            game_data: GameData {
                name: "Toontown Corporate Clash".to_string(),
                available_platforms: vec![Platforms::Windows, Platforms::Mac],
                is_installed: false,
            },
            installer_path,
            args,
        }
    }
}

impl Game for TTCC {
    fn download_game(&mut self, download_location: PathBuf, install_location: PathBuf) -> Result<(), String> {
        todo!()
    }

    fn update_game(&self, download_location: PathBuf, install_location: PathBuf) -> Result<(), String> {
        todo!()
    }

    fn launch_game(&self, username: String, password: String, install_location: PathBuf) -> Result<(), String> {
        todo!()
    }
}
