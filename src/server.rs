use std::env;
use std::fs::{self, DirEntry, Metadata};
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug)]
pub struct FileContent {
    pub path: PathBuf,
    pub frontmatter: Frontmatter,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct Frontmatter {
    title: String,
    date: String,
    draft: bool,
}

pub fn server() -> Result<(), Box<dyn std::error::Error>> {
    build()?;

    Ok(())
}

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building site...");

    // Check if we're in an actual site directory, else panic
    let mut path = env::current_dir()?;
    let mut toml_path = path.clone();

    toml_path.push("config.toml");
    if toml_path.exists() {
        path.push("content");
    } else {
        return Err(
            "Not inside a site. Have you changed directory to your site? Can't build site".into(),
        );
    }

    // Walk directory and subdirectories and get the files we need to parse to build the site
    let entries = walk_dir(&path);

    for entry in entries {
        if let Ok(file_content) = parse_file(&entry) {
            println!("{:?}", file_content.path);
            println!(
                "Frontmatter: {}, {}, {}",
                file_content.frontmatter.title,
                file_content.frontmatter.date,
                file_content.frontmatter.draft
            );
            println!("Body: {}", file_content.body);
        } else {
            eprintln!("Warning: skipping {:?} - Parsing failed.", entry.path());
        }
    }

    Ok(())
}

pub fn parse_file(file: &DirEntry) -> Result<FileContent, Box<dyn std::error::Error>> {
    println!("Parsing {:?}", &file.file_name());
    let contents = fs::read_to_string(&file.path())?;
    let frontmatter = match parse_frontmatter(&contents) {
        Ok(cont) => cont,
        Err(err) => {
            eprintln!(
                "Error when parsing frontmatter: {}. File has no frontmatter?",
                err
            );
            return Err(err);
        }
    };
    let body = match parse_body(&contents) {
        Ok(body) => body,
        Err(err) => {
            eprintln!("Error when parsing file body: {}. File is empty?", err);
            return Err(err);
        }
    };
    Ok(FileContent {
        path: file.path(),
        frontmatter,
        body,
    })
}

pub fn parse_body(file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body: String = file
        .lines()
        .skip_while(|line| line.trim() != "---")
        .skip(1)
        .skip_while(|line| !line.trim().contains("---"))
        .skip(1)
        .collect::<Vec<_>>()
        .join("\n");
    Ok(body)
}

pub fn parse_frontmatter(file: &str) -> Result<Frontmatter, Box<dyn std::error::Error>> {
    let fmttext: String = file
        .lines()
        .skip_while(|line| line.trim() != "---")
        .skip(1)
        .take_while(|line| line.trim() != "---")
        .collect::<Vec<_>>()
        .join("\n");
    let frontmatter: Frontmatter = serde_yaml::from_str(&fmttext)?;

    Ok(frontmatter)
}

// Main recutrsive function that goes through the "parent" content directory
// and all subdirectories.
pub fn walk_dir(directory: &PathBuf) -> Vec<DirEntry> {
    let mut files = Vec::new();
    fs::read_dir(&directory)
        .into_iter()
        .flatten()
        .flatten()
        .for_each(|entry| {
            if entry.metadata().as_ref().is_ok_and(Metadata::is_file) {
                files.push(entry);
            } else if entry.metadata().as_ref().is_ok_and(Metadata::is_dir) {
                let subdirectory_files = walk_dir(&entry.path());
                files.extend(subdirectory_files);
            }
        });

    files
}
