use std::fs;
use std::io;
use zip::read::ZipArchive;
use crate::error::{AppError, AppResult};

pub fn extract_zip(zip_file_name: &str) -> AppResult<()> {
    println!("Extracting repository...");
    let zip_file = fs::File::open(zip_file_name).map_err(|e| AppError::Io(e))?;
    let mut archive = ZipArchive::new(zip_file).map_err(|e| AppError::Zip(e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| AppError::Zip(e))?;
        let outpath = file.mangled_name();

        if file.is_dir() {
            fs::create_dir_all(&outpath).map_err(|e| AppError::Io(e))?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).map_err(|e| AppError::Io(e))?;
                }
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| AppError::Io(e))?;
            io::copy(&mut file, &mut outfile).map_err(|e| AppError::Io(e))?;
        }
    }
    println!("Extraction complete.");
    Ok(())
}
