use std::collections::HashMap;

// Define a struct to represent a file
#[derive(Debug, Clone)]
pub struct File {
    pub file_id: String,
    pub name: String,
    pub content: Vec<u8>,
    pub metadata: HashMap<String, String>,
}

// Function to create a new file
pub fn create_file(file_id: &str, name: &str, content: Vec<u8>) -> File {
    File {
        file_id: file_id.to_string(),
        name: name.to_string(),
        content,
        metadata: HashMap::new(),
    }
}

// Function to add metadata to a file
pub fn add_metadata_to_file(file: &mut File, key: &str, value: &str) {
    file.metadata.insert(key.to_string(), value.to_string());
}

// Function to update the content of a file
pub fn update_file_content(file: &mut File, new_content: Vec<u8>) {
    file.content = new_content;
}

// Example usage
fn main() {
    let mut file = create_file("file-12345678", "Document.txt", vec![72, 101, 108, 108, 111]); // "Hello" in ASCII
    println!("Created File: {:?}", file);

    add_metadata_to_file(&mut file, "author", "user123");
    println!("Updated File Metadata: {:?}", file);

    update_file_content(&mut file, vec![87, 111, 114, 108, 100]); // "World" in ASCII
    println!("Updated File Content: {:?}", file);
}