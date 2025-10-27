use std::{
    env,
    fmt::Display,
    fs,
    io::{BufWriter, Write},
};

// Holds the configuration of the site that will also be used in config.toml
pub struct Config {
    base_url: String,
    language_code: String,
    title: String,
}

impl Config {
    // Creates new configuration with default info
    pub fn new() -> Config {
        Config {
            base_url: String::from("https://example.org/"),
            language_code: String::from("en-us"),
            title: String::from("My New Rusite Site"),
        }
    }

    pub fn write_initial_toml(&self, file: &fs::File) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf_writer = BufWriter::new(file);
        buf_writer.write_all(self.to_string().as_bytes())?;

        Ok(())
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "baseURL = \'{}\'\nlanguageCode = \'{}\'\ntitle = \'{}\'\n",
            self.base_url, self.language_code, self.title
        )
    }
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
    let config_file = fs::File::create("config.toml")?;

    // Write minimum elements of new toml
    let config = Config::new();
    config.write_initial_toml(&config_file)?;

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
    println!("3. Start the embedded web server with the command \"rusite server --buildDrafts\".");
}
