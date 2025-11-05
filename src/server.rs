use std::env;
use std::fs::{self, DirEntry, Metadata};
use std::path::PathBuf;

pub fn server() -> Result<(), Box<dyn std::error::Error>> {
    build()?;

    Ok(())
}

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building site...");

    // Check if we're in an actual site directory
    let mut path = env::current_dir()?;
    let mut toml_path = path.clone();

    toml_path.push("config.toml");
    if toml_path.exists() {
        println!("We are in a site directory, I think.");
        path.push("content");
    } else {
        return Err(
            "Not inside a site. Have you changed directory to your site? Can't build site".into(),
        );
    }

    walk_dir(&path);

    Ok(())
}

#[allow(unused_variables)]
pub fn walk_dir(directory: &PathBuf) -> Vec<DirEntry> {
    let mut files = Vec::new();

    // fs::read_dir(&directory)
    //     .into_iter()
    //     .flatten()
    //     .flatten()
    //     .filter(|entry| entry.metadata().as_ref().is_ok_and(Metadata::is_file))
    //     .for_each(|entry| files.push(entry));

    fs::read_dir(&directory)
        .into_iter()
        .flatten()
        .flatten()
        .for_each(|entry| {
            if entry.metadata().as_ref().is_ok_and(Metadata::is_file) {
                println!("File: {:?}", &entry);
                files.push(entry);
            } else if entry.metadata().as_ref().is_ok_and(Metadata::is_dir) {
                println!("Directory: {:?}", &entry);
                let subdirectory_files = walk_dir(&entry.path());
                files.extend(subdirectory_files);
            }
        });

    files
}
