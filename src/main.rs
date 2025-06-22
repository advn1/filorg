use directories::UserDirs;
use std::fs;
use std::fs::read_dir;
use std::io::{Error, ErrorKind};

fn main() {
    if let Err(e) = organize() {
        println!("IO error: {:?}", e);
    }
}

fn organize() -> Result<(), Error> {
    let dirs = UserDirs::new()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "User's directories were not found"))?;

    let downloads_dir = dirs.download_dir().ok_or_else(|| {
        Error::new(
            ErrorKind::NotFound,
            "User's Downloads directory was not found",
        )
    })?;
    let desktop_dir = dirs.desktop_dir().ok_or_else(|| {
        Error::new(
            ErrorKind::NotFound,
            "User's Desktop directory was not found",
        )
    })?;
    let mut files = read_dir(downloads_dir)?;

    while let Some(f) = files.next() {
        let file = f?;

        let full_path = file.path();
        println!("{:?}", full_path.extension());

        if let Some(ext) = full_path.extension().and_then(|s| s.to_str()) {
            match ext {
                "pdf" | "jpg" | "jpeg" | "png" | "zip" => {
                    let dest_dir = desktop_dir.join(format!("{}_files", ext));

                    fs::create_dir_all(&dest_dir)?;

                    let file_name = full_path.file_name().ok_or_else(|| {
                        Error::new(ErrorKind::InvalidInput, "Could not get file name")
                    })?;

                    let dest_file = dest_dir.join(file_name);

                    println!(
                        "Copying {} to {}...",
                        full_path.display(),
                        dest_file.display()
                    );

                    fs::rename(&full_path, &dest_file)?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
