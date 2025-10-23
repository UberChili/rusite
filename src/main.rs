use std::{env, fs, process};

use clap::{Parser, Subcommand};

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

fn create_site(name: &str) -> Result<(), Box<dyn std::error::Error>> {
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

fn check_valid_archetype(name: &str) -> bool {
    name.ends_with(".md")
}

fn create_content(name: &str) -> Result<(), Box<dyn std::error::Error>> {
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
            "Not inside a site. Have you changed directory to your site? Can't create content.".into(),
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
    println!("Your new content \"{}\" was created!", &name);

    Ok(())
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Command
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    New {
        #[command(subcommand)]
        target: NewTarget,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum NewTarget {
    Site {name: String},
    Content {name: String},
}


fn main() {
    let args = Args::parse();

    if let Err(err) = run(args) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }

}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {

    match args.cmd {
        Commands::New { target } => {
            match target {
                NewTarget::Site { name } => create_site(&name)?,
                NewTarget::Content { name } => create_content(&name)?,
            }
        }
    };

    Ok(())
}