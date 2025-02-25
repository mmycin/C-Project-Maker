use std::fs;
use std::io;
use zip::read::ZipArchive;

pub fn extract_zip(zip_file_name: &str) -> io::Result<()> {
    println!("Extracting repository...");
    let zip_file = fs::File::open(zip_file_name)?;
    let mut archive = ZipArchive::new(zip_file).expect("Failed to read ZIP archive");

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
    Ok(())
}
