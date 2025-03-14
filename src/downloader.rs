use std::fs;
use reqwest::blocking::get;
use crate::error::{AppError, AppResult};

pub fn download_zip(url: &str, output_file: &str) -> AppResult<()> {
    println!("Downloading repository...");
    let response = get(url).map_err(|e| AppError::Download(e))?;
    
    if !response.status().is_success() {
        return Err(AppError::Download(response.error_for_status().unwrap_err()));
    }
    
    let bytes = response.bytes().map_err(|e| AppError::Download(e))?;
    
    // Validate that the downloaded content looks like a ZIP file
    if bytes.len() < 4 || &bytes[0..4] != b"PK\x03\x04" {
        return Err(AppError::Config("Downloaded file is not a valid ZIP archive".to_string()));
    }
    
    fs::write(output_file, &bytes).map_err(|e| AppError::Io(e))?;
    println!("Download complete.");
    Ok(())
}
