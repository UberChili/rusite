use crate::parser::parse_file;
use crate::parser::walk_dir;
use std::env;

pub fn server() -> Result<(), Box<dyn std::error::Error>> {
    build()?;

    Ok(())
}

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building site...");

    // Check if we're in an actual site directory, else panic
    let mut path = env::current_dir()?;
    let mut toml_path = path.clone();

    toml_path.push("config.toml");
    if toml_path.exists() {
        path.push("content");
    } else {
        return Err(
            "Not inside a site. Have you changed directory to your site? Can't build site".into(),
        );
    }

    // Walk directory and subdirectories and get the files we need to parse to build the site
    let entries = walk_dir(&path);

    for entry in entries {
        if let Ok(file_content) = parse_file(&entry) {
            println!("{:?}", file_content.path);
            println!(
                "Frontmatter: {}, {}, {}",
                file_content.frontmatter.title,
                file_content.frontmatter.date,
                file_content.frontmatter.draft
            );
            println!("Body: {}", file_content.body);
        } else {
            eprintln!("Warning: skipping {:?} - Parsing failed.", entry.path());
        }
    }

    Ok(())
}
