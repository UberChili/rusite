use std::env;

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
    let cwd = env::current_dir().unwrap();

    match what {
        What::Site => {
            println!(
                // Fix to display the actual path, not cwd
                "Congratulations! Your new site \"{}\", was created in: {:?}.",
                name,
                cwd.display()
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
        What::Post => {
            println!("Your new post \"{}\" was created!", &name);
        }
    }
}

enum What {
    Site,
    Post,
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
            } else if what == "post" {
                // prompt for post
                initial_prompt(&What::Post, &name);
            }
        }
    };
}
