use super::game::{Game, GameData};
use std::path::PathBuf;

const MANIFEST_PATH: &str = "https://cdn.toontownrewritten.com/content/patchmanifest.txt";
const CDN_PATH: &str = "https://download.toontownrewritten.com/patches/";

pub struct TTR {
    game_data: GameData,
    args: Vec<String>,
}

impl Game for TTR {
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
