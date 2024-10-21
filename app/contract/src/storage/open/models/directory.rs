use std::collections::HashMap;

// Define a struct to represent a directory
#[derive(Debug, Clone)]
pub struct Directory {
    pub directory_id: String,
    pub name: String,
    pub files: Vec<String>, // List of file IDs
    pub metadata: HashMap<String, String>,
}

// Function to create a new directory
pub fn create_directory(directory_id: &str, name: &str) -> Directory {
    Directory {
        directory_id: directory_id.to_string(),
        name: name.to_string(),
        files: Vec::new(),
        metadata: HashMap::new(),
    }
}

// Function to add a file to a directory
pub fn add_file_to_directory(directory: &mut Directory, file_id: &str) {
    directory.files.push(file_id.to_string());
}

// Function to add metadata to a directory
pub fn add_metadata_to_directory(directory: &mut Directory, key: &str, value: &str) {
    directory.metadata.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut directory = create_directory("dir-12345678", "Documents");
    println!("Created Directory: {:?}", directory);

    add_file_to_directory(&mut directory, "file-123");
    add_metadata_to_directory(&mut directory, "owner", "user123");
    println!("Updated Directory: {:?}", directory);
}