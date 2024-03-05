use super::game::{Game, GameData};
use std::path::PathBuf;

const INSTALLER_PATH: &str =
    "https://github.com/CorporateClash/pyside2-releases/releases/download/v1.3.0/installer.exe";

pub struct TTCC {
    game_data: GameData,
    installer_path: PathBuf,
    args: Vec<String>,
}

impl Game for TTCC {
    fn download_game(&self, download_location: PathBuf) -> Result<(), String> {
        todo!()
    }

    fn install_game(&self, install_location: PathBuf) -> Result<(), String> {
        todo!()
    }

    fn update_game(&self, installed_game_location: PathBuf) -> Result<(), String> {
        todo!()
    }

    fn launch_game(&self) -> Result<(), String> {
        todo!()
    }
}
