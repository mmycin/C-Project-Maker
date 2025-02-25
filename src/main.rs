use std::env;
use std::fs;
use std::path::Path;
use std::io::{self};
use reqwest::blocking::get;
use zip::read::ZipArchive;

fn main() -> io::Result<()> {
    // Check if folder name is provided as an argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <folder-name>");
        return Ok(());
    }
    let folder_name = &args[1];

    // Define repository URL and ZIP file name
    let repo_url = "https://github.com/mmycin/C-Template/archive/refs/heads/master.zip";
    let zip_file_name = "repo.zip";

    // Step 1: Download repository ZIP
    println!("Downloading repository...");
    let response = get(repo_url).expect("Failed to download file");
    let bytes = response.bytes().expect("Failed to read response bytes");
    fs::write(zip_file_name, &bytes).expect("Failed to save zip file");
    println!("Download complete.");

    // Step 2: Extract the ZIP file
    println!("Extracting repository...");
    let zip_file = fs::File::open(zip_file_name)?;
    let mut archive = ZipArchive::new(zip_file).expect("Failed to read ZIP archive");

    let extracted_folder = "C-Template-master"; // Folder inside the ZIP
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to access ZIP entry");
        let outpath = file.mangled_name();

        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    println!("Extraction complete.");

    // Step 3: Rename extracted folder
    let new_folder_name = Path::new(folder_name);
    if new_folder_name.exists() {
        println!("Folder '{}' already exists. Deleting it...", folder_name);
        fs::remove_dir_all(new_folder_name)?;
    }

    println!("Renaming folder from '{}' to '{}'...", extracted_folder, folder_name);
    fs::rename(extracted_folder, new_folder_name)?;
    println!("Folder renamed.");

    // Step 4: Delete ZIP file
    println!("Deleting zip file...");
    fs::remove_file(zip_file_name)?;
    println!("Zip file deleted.");

    println!("Process completed successfully!");
    Ok(())
}
