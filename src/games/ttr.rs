use super::game::{Game, GameData};
use std::path::PathBuf;
use serde_json::Value;

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


/// Fetches the manifest from `MANIFEST_PATH` and returns it as a `serde_json::Value`.
fn get_manifest() -> Result<Value, String> {
    let manifest = match reqwest::blocking::get(MANIFEST_PATH) {
        Ok(manifest) => manifest,
        Err(_) => return Err("Could not get manifest".to_string()),
    };
    let manifest_text = match manifest.text() {
        Ok(manifest_text) => manifest_text,
        Err(_) => return Err("Could not read manifest".to_string()),
    };
    match serde_json::from_str(&manifest_text) {
        Ok(manifest_json) => manifest_json,
        Err(_) => return Err("Could not parse manifest".to_string()),
    }
}


/// Goes through the manifest file (located in `MANIFEST_FILE`), updating existing files and downloading new ones
fn update_or_download_gamefiles() -> Result<(), String> {
    let manifest = get_manifest()?;
    Ok(())
}
