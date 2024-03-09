use super::game::{Game, GameData, Platforms};
use super::utils::{download_new_file_and_get_path, extract_bzip2};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, ACCEPT, CONTENT_TYPE},
};
use serde_json::{json, Value};
use std::fs;
use std::io::stdin;
use std::path::PathBuf;
use std::process::Command;

const MANIFEST_PATH: &str = "https://cdn.toontownrewritten.com/content/patchmanifest.txt";
const CDN_PATH: &str = "https://download.toontownrewritten.com/patches/";
const LOGIN_PATH: &str = "https://www.toontownrewritten.com/api/login?format=json";

pub struct TTR {
    game_data: GameData,
    args: Vec<String>,
}

impl TTR {
    pub fn new(args: Vec<String>) -> TTR {
        TTR {
            game_data: GameData {
                name: "Toontown Rewritten".to_string(),
                available_platforms: vec![Platforms::Windows, Platforms::Mac, Platforms::Linux],
                is_installed: false,
            },
            args,
        }
    }
    fn login_to_game(&self, username: String, password: String) -> Result<Value, String> {
        let client = Client::new();
        let credentials = vec![("username", username), ("password", password)];
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded".parse().unwrap(),
        );
        let response = client
            .post(LOGIN_PATH)
            .headers(headers.clone())
            .form(&credentials)
            .send()
            .unwrap()
            .text()
            .unwrap();
        let response_json: Value = serde_json::from_str(&response).unwrap();
        println!("{:?}", response_json.to_string());
        match response_json["success"].as_str().unwrap() {
            "true" => Ok(response_json),
            "false" => Err(response_json["banner"].to_string()),
            "partial" => handle_tfa(
                response_json["responseToken"].to_string(),
                client,
                headers,
                &credentials,
            ),
            "delayed" => Err("Delayed login".to_string()),
            _ => Err("Unknown error".to_string()),
        }
    }
}

impl Game for TTR {
    fn download_game(
        &mut self,
        download_location: PathBuf,
        install_location: PathBuf,
    ) -> Result<(), String> {
        update_or_download_gamefiles(download_location, install_location)?;
        self.game_data.is_installed = true;
        Ok(())
    }
    fn update_game(
        &self,
        download_location: PathBuf,
        install_location: PathBuf,
    ) -> Result<(), String> {
        update_or_download_gamefiles(download_location, install_location)
    }

    fn launch_game(
        &self,
        username: String,
        password: String,
        install_location: PathBuf,
    ) -> Result<(), String> {
        let login_token = self.login_to_game(username, password)?;
        let mut command = Command::new(install_location.join("TTREngine64.exe"));
        let cookie_parameter = login_token["cookie"].to_string().replace("\"", "");
        let gameserver_parameter = login_token["gameserver"].to_string().replace("\"", "");
        command
            .env("TTR_PLAYCOOKIE", &cookie_parameter)
            .env("TTR_GAMESERVER", &gameserver_parameter)
            .current_dir(install_location)
            .output()
            .unwrap();
        Ok(())
    }
}

/// Fetches the manifest from `MANIFEST_PATH` and returns it as a `serde_json::Value`.
fn get_manifest() -> Result<Value, String> {
    let manifest = reqwest::blocking::get(MANIFEST_PATH).unwrap();
    let manifest_text = manifest.text().unwrap();
    serde_json::from_str(&manifest_text).map_err(|e| e.to_string())
}

/// Goes through the manifest file (located in `MANIFEST_FILE`), updating existing files and downloading new ones
fn update_or_download_gamefiles(
    download_path: PathBuf,
    install_path: PathBuf,
) -> Result<(), String> {
    let manifest = get_manifest()?;
    let reqwest_client = Client::new();
    for (filename, file_metadata) in manifest.as_object().unwrap() {
        let bz2_path = download_new_file_and_get_path(
            &download_path,
            file_metadata["dl"].as_str().unwrap(),
            &reqwest_client,
            CDN_PATH,
        )?;
        extract_bzip2(&bz2_path, &install_path.join(filename))?;
        fs::remove_file(bz2_path).unwrap();
    }
    Ok(())
}

fn handle_tfa(
    response_token: String,
    client: Client,
    headers: HeaderMap,
    credentials: &Vec<(&str, String)>,
) -> Result<Value, String> {
    let mut tfa_token = String::new();
    println!("Additional authentication needed. Enter your 2FA token: ");
    stdin().read_line(&mut tfa_token).unwrap();
    let response = client
        .post(LOGIN_PATH)
        .headers(headers.clone())
        .form(&credentials)
        .send()
        .unwrap()
        .text()
        .unwrap();
    let response_json: Value = serde_json::from_str(&response).unwrap();
    match response_json["success"].as_str().unwrap() {
        "true" => Ok(response_json),
        "false" => Err(response_json["banner"].to_string()),
        "partial" => {
            println!("Incorrect token, try again.");
            handle_tfa(response_token, client, headers, credentials)
        }
        "delayed" => Err("Delayed login".to_string()),
        _ => Err("Unknown error".to_string()),
    }
}
