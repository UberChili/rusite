use std::{env, fs};

pub fn check_valid_archetype(name: &str) -> bool {
    name.ends_with(".md")
}

// Creates a new site
pub fn create_site(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut path = env::current_dir()?;
    path.push(&name);
    if path.exists() {
        let err = format!("Site {} already exists!", &name);
        return Err(err.into());
    }
    fs::create_dir_all(&path)?;
    env::set_current_dir(&path)?;
    fs::create_dir("content")?;
    fs::create_dir("static")?;
    fs::create_dir("layouts")?;
    fs::create_dir("themes")?;
    fs::create_dir("archetypes")?;
    fs::create_dir("assets")?;
    fs::create_dir("data")?;
    fs::File::create("config.toml")?;

    new_site_msg(&name);
    Ok(())
}

fn new_site_msg(name: &str) {
    println!("Congratulations! Your new site \"{}\", was created!", &name);
    println!("\nJust a few more steps... \n");
    println!("1. Change the current directory to {}/", &name);
    println!(
        "2. Create new content with the command \"rusite new content <SECTIONNAME>/<FILENAME>.<FORMAT>\"."
    );
    println!(
        "3. Start the embedded web server with the command \"rusite server --buildDrafts\"."
    );
}