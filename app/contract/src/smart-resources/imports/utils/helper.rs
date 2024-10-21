use std::fs;
use std::io;
use std::path::Path;

// Function to check if a file exists
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

// Function to read a file and return its contents as a string
pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

// Function to validate the format of a file based on its extension
pub fn validate_file_format<P: AsRef<Path>>(path: P, allowed_formats: &[&str]) -> bool {
    if let Some(extension) = path.as_ref().extension() {
        if let Some(ext_str) = extension.to_str() {
            return allowed_formats.contains(&ext_str);
        }
    }
    false
}

// Example usage
fn main() -> io::Result<()> {
    let file_path = "imports/services.csv";

    if !file_exists(file_path) {
        println!("File does not exist.");
        return Ok(());
    }

    if !validate_file_format(file_path, &["csv", "json"]) {
        println!("File format is not allowed.");
        return Ok(());
    }

    let content = read_file_to_string(file_path)?;
    println!("File Content: {}", content);

    Ok(())
}