use crate::check_valid_archetype;
use std::{
    env,
    error::Error,
    fs,
    io::{BufWriter, Write},
    path::PathBuf,
};

#[derive(Debug)]
pub enum Target {
    Post,
}

pub fn frontmatter(target: Target, path: &PathBuf, name: &str) -> Result<(), Box<dyn Error>> {
    match target {
        Target::Post => {
            let title: String = format!("title: {}", &name);
            let file = fs::File::create(&path)?;
            let mut buf_writer = BufWriter::new(file);
            buf_writer.write_all("---\n".as_bytes())?;
            buf_writer.write_all(title.as_bytes())?;
            buf_writer.write_all("---\n".as_bytes())?;
        }
    };

    Ok(())
}

pub fn create_content(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !check_valid_archetype(&name) {
        return Err("Error: Failed to resolve \"{name}\" to an archetype template".into());
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

    fs::File::create(&path)?;
    frontmatter(Target::Post, &path, &name)?;
    println!("Your new content \"{}\" was created!", &name);

    Ok(())
}
