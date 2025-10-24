use std::process;

use clap::{Parser, Subcommand};

use rusite::{site, content};

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
                NewTarget::Site { name } => site::create_site(&name)?,
                NewTarget::Content { name } => content::create_content(&name)?,
            }
        }
    };

    Ok(())
}