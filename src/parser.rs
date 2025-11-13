use pulldown_cmark::{html, Parser};
use serde::Deserialize;
use std::env;
use std::fs::{self, DirEntry, Metadata};
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileContent {
    pub path: PathBuf,
    pub frontmatter: Frontmatter,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    pub draft: bool,
}

pub fn parse_file(file: &DirEntry) -> Result<FileContent, Box<dyn std::error::Error>> {
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

// Converts markdown contents of a file to html
pub fn markdown_to_html(file: &FileContent) -> String {
    let parser = Parser::new(&file.body);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn write_to_html(file: &FileContent, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    // check if public directory exists, if not, create it
    let mut path = env::current_dir()?;
    path.push("public/");
    path.push("posts/");
    path.push(&file.frontmatter.title);
    fs::create_dir_all(&path)?;

    path.push("index.html");

    // Write to file
    let html_file = fs::File::create(path)?;
    let mut buf_writer = BufWriter::new(html_file);
    buf_writer.write_all(body.as_bytes())?;

    Ok(())
}
