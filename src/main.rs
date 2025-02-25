mod downloader;
mod extractor;
mod utils;

use std::env;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <folder-name>");
        return Ok(());
    }
    let folder_name = &args[1];

    let zip_file_name = "repo.zip";
    let repo_url = "https://github.com/mmycin/C-Template/archive/refs/heads/master.zip";

    downloader::download_zip(repo_url, zip_file_name)?;
    extractor::extract_zip(zip_file_name)?;

    utils::rename_folder("C-Template-master", folder_name)?;
    utils::delete_file(zip_file_name)?;

    println!("Process completed successfully!");
    Ok(())
}
