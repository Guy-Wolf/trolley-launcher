use bzip2::read::BzDecoder;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn download_new_file_and_get_path(
    download_path: &PathBuf,
    filename: &str,
    reqwest_client: &Client,
    cdn_path: &str,
) -> Result<PathBuf, String> {
    println!("Downloading {:?}...", filename);
    let mut file = std::fs::File::create(download_path.join(&filename)).unwrap();
    let response = reqwest_client
        .get(format!("{}{}", cdn_path, &filename))
        .send()
        .unwrap();
    file.write_all(&response.bytes().unwrap()).unwrap();
    Ok(PathBuf::from(download_path.join(&filename)))
}

pub fn extract_bzip2(bz2_file_path: &PathBuf, output_path: &PathBuf) -> Result<(), String> {
    let bz2_file = File::open(&bz2_file_path).unwrap();
    let mut decoder = BzDecoder::new(bz2_file);
    let mut output_file =
        File::create(output_path).unwrap();
    io::copy(&mut decoder, &mut output_file).unwrap();
    Ok(())
}
