
pub mod site;
pub mod content;

// pub use site::create_site;
// pub use content::create_content;

pub fn check_valid_archetype(name: &str) -> bool {
    name.ends_with(".md")
}

