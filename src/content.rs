use chrono::{DateTime, Local};

use crate::check_valid_archetype;
use std::{
    env,
    error::Error,
    fmt::Display,
    fs,
    io::{BufWriter, Write},
    path::PathBuf,
};

#[derive(Debug)]
pub enum Target {
    Post,
}

#[derive(Debug)]
pub struct FrontMatterInfo {
    title: String,
    draft: bool,
    date: DateTime<Local>,
}

impl FrontMatterInfo {
    pub fn new(name: &str, date: &DateTime<Local>) -> FrontMatterInfo {
        FrontMatterInfo {
            title: name.to_string(),
            draft: true,
            date: *date,
        }
    }
}

impl Display for FrontMatterInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "---\ntitle: {}\ndate: {}\ndraft: {}\n---\n",
            self.title,
            self.date.to_rfc3339(),
            self.draft,
        )
    }
}

pub fn frontmatter(
    file: &fs::File,
    target: Target,
    path: &PathBuf,
    name: &str,
) -> Result<(), Box<dyn Error>> {
    match target {
        Target::Post => {
            let date = chrono::Local::now();
            let title = path.file_stem().and_then(|s| s.to_str()).unwrap_or(name);
            let ftmatter = FrontMatterInfo::new(&title, &date);
            let mut buf_writer = BufWriter::new(file);
            // buf_writer.write_all(ftmatter.to_string().as_bytes())?;
            buf_writer.write_all(ftmatter.to_string().as_bytes())?;
        }
    };

    Ok(())
}

pub fn create_content(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !check_valid_archetype(&name) {
        let err = format!(
            "Error: Failed to resolte \"{}\" to an archetype template",
            &name
        );
        return Err(err.into());
    }

    let mut path = env::current_dir()?;

    // Check if we're in an actual site directry
    let mut toml_path = path.clone();
    toml_path.push("config.toml");
    if toml_path.exists() {
        println!("We can kinda think we are inside an actual site directory. Creating post...")
    } else {
        return Err(
            "Not inside a site. Have you changed directory to your site? Can't create content."
            .into(),
        );
    }

    path.push("content");

    // Check if directory (or content type) was provided and act accordingly
    if name.contains("/") {
        if let Some((content_dir, post_name)) = name.rsplit_once("/") {
            path.push(content_dir);
            if !path.exists() {
                fs::create_dir(&path)?;
            }
            path.push(post_name);
        }
    } else {
        if !path.exists() {
            fs::create_dir(&path)?;
        }
        path.push(name);
    }

    let file = fs::File::create(&path)?;
    frontmatter(&file, Target::Post, &path, &name)?;
    println!("Your new content \"{}\" was created!", &name);

    Ok(())
}
