use std::fs;
use std::io;
use std::path::Path;

pub fn rename_folder(old_name: &str, new_name: &str) -> io::Result<()> {
    let new_folder_path = Path::new(new_name);
    if new_folder_path.exists() {
        println!("Folder '{}' already exists. Deleting it...", new_name);
        fs::remove_dir_all(new_folder_path)?;
    }

    println!("Renaming folder from '{}' to '{}'...", old_name, new_name);
    fs::rename(old_name, new_folder_path)?;
    println!("Folder renamed.");
    Ok(())
}

pub fn delete_file(file_path: &str) -> io::Result<()> {
    println!("Deleting {}...", file_path);
    fs::remove_file(file_path)?;
    println!("File deleted.");
    Ok(())
}
