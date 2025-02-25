use std::fs;
use std::io;
use reqwest::blocking::get;

pub fn download_zip(url: &str, output_file: &str) -> io::Result<()> {
    println!("Downloading repository...");
    let response = get(url).expect("Failed to download file");
    let bytes = response.bytes().expect("Failed to read response bytes");
    fs::write(output_file, &bytes).expect("Failed to save zip file");
    println!("Download complete.");
    Ok(())
}
