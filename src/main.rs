use std::{env, fs};

use clap::{Parser, Subcommand};

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
        what_to_create: String,
        name: String,
    },
}

fn initial_prompt(what: &What, name: &String) {
    match what {
        What::Site => {
            match create_site(&name) {
                Ok(_) => {
                    println!(
                        // Fix to display the actual path, not cwd
                        "Congratulations! Your new site \"{}\", was created!",
                        name
                    );

                    println!("\nJust a few more steps...\n");

                    // Placeholder still
                    println!("1. Change the current directory to {}", "directory.");
                    println!(
                        "2. Create new content with the command \"rusite new content <SECTIONNAME>/<FILENAME>.<FORMAT>\"."
                    );
                    println!(
                        "3. Start the embedded web server with the command \"rusite server --buildDrafts\"."
                    );
                }
                Err(err) => eprintln!("{err}"),
            };
        }
        What::Post => {
            match create_content(&name) {
                Ok(_) => println!("Your new post \"{}\" was created!", &name),
                Err(err) => eprintln!("Could not create content: {err}"),
            };
        }
    }
}

enum What {
    Site,
    Post,
}

fn create_site(name: &String) -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}

fn check_valid_archetype(name: &String) -> bool {
    if name.ends_with(".md") {
        return true;
    }
    false
}

fn create_content(name: &String) -> Result<(), Box<dyn std::error::Error>> {
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
    println!("File succesfully created!");
    Ok(())
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::New {
            what_to_create: what,
            name,
        } => {
            if what == "site" {
                // prompt for site
                initial_prompt(&What::Site, &name);
                // Todo hurr we haven't created a site yet
            } else if what == "post" {
                // prompt for post
                initial_prompt(&What::Post, &name);
                // let _ = create_content(&name);
            }
        }
    };
}
