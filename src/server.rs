use std::fs::{self, DirEntry};
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
pub fn walk_dir(directory: &PathBuf) {
    if let Ok(entries) = fs::read_dir(&directory) {
        for entry in entries {
            println!("{:?}", entry.unwrap());
        }
    }
}
