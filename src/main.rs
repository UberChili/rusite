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
            match create_post(&name) {
                Ok(_) => println!("Your new post \"{}\" was created!", &name),
                Err(err) => eprintln!("{err}"),
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

fn create_post(name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("{}.md", name);
    let mut path = env::current_dir()?;
    let mut toml_path = path.clone();
    toml_path.push("config.toml");

    if toml_path.exists() {
        println!("We can kinda think we are inside an actual site directory. Creating post...")
    } else {
        return Err(
            "Not inside a site. Have you change directory to your site? Can't create post.".into(),
        );
    }

    println!("Trying to create post with name {}.md", name);

    path.push("content");
    path.push("posts");
    if !path.exists() {
        fs::create_dir(&path)?;
    }
    path.push(filename);

    fs::File::create(&path).expect("Could not create file.");
    println!("At path: {:?}", &path);
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
                // let _ = create_post(&name);
            }
        }
    };
}
