use std::{
    env,
    fs::{self, DirEntry},
    path::PathBuf,
};

pub fn server() -> Result<(), Box<dyn std::error::Error>> {
    build()?;

    Ok(())
}

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building site...");

    // Check if we're in an actual site directory
    let path = env::current_dir()?;
    let mut toml_path = path.clone();

    toml_path.push("config.toml");
    if toml_path.exists() {
        println!("We are in a site directory, I think.")
    } else {
        return Err(
            "Not inside a site. Have you changed directory to your site? Can't build site".into(),
        );
    }

    // Check all files to read
    // currently we should just focus on the immediate contents of content/
    let mut files: Vec<DirEntry> = Vec::new();

    for entry in fs::read_dir(&path)? {
        files.push(entry?);
    }

    Ok(())
}
