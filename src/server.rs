use std::fs::{self, DirEntry, Metadata};
use std::path::PathBuf;
use std::{
    env,
    // fs::{self, DirEntry}, path::PathBuf,
};

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

    // Check all files to read
    // currently we should just focus on the immediate contents of content/
    // let mut entries: Vec<DirEntry> = Vec::new();

    // for entry in fs::read_dir(&path)? {
    //     entries.push(entry?);
    // }

    // for entry in entries {
    //     println!("{:?}", entry);
    //     println!("{:?}", entry.path());
    // }

    // if let Ok(entries) = fs::read_dir(&path) {
    //     // for entry in entries {
    //     //     if let Ok(entry) = entry {
    //     //         println!("{:?} type is: {:?}", &entry.file_name(), &entry.file_type());
    //     //     }
    //     // }
    // }

    walk_dir(&path);

    Ok(())
}

#[allow(unused_variables)]
pub fn walk_dir(directory: &PathBuf) -> Vec<DirEntry> {
    // TODO fix this or convert to idiomatic Rust

    // let mut entries_v: Vec<DirEntry> = Vec::new();

    // if let Ok(entries) = fs::read_dir(&directory) {
    //     for entry in entries {
    //         if let Ok(direntry) = entry {
    //             if let Ok(metadata) = direntry.metadata() {
    //                 if metadata.is_file() {
    //                     println!("{:?} is a file", direntry.file_name());
    //                 }
    //             }
    //         }
    //     }
    // }

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
                walk_dir(&entry.path());
            }
        });

    files
}
