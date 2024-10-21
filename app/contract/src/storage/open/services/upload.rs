use std::collections::HashMap;
use crate::storage::open::models::file::File;
use crate::storage::open::models::directory::Directory;

// Function to upload a file to a directory
pub fn upload_file(directory: &mut Directory, file: File, files: &mut HashMap<String, File>) {
    files.insert(file.file_id.clone(), file.clone());
    directory.files.push(file.file_id.clone());
    println!("Uploaded file {} to directory {}", file.name, directory.name);
}

// Function to update an existing file's content
pub fn update_file_content(file_id: &str, new_content: Vec<u8>, files: &mut HashMap<String, File>) -> bool {
    if let Some(file) = files.get_mut(file_id) {
        file.content = new_content;
        println!("Updated content of file {}", file.name);
        true
    } else {
        println!("File {} not found", file_id);
        false
    }
}

// Example usage
fn main() {
    let mut files = HashMap::new();
    let mut directory = Directory {
        directory_id: "dir-123".to_string(),
        name: "Documents".to_string(),
        files: vec![],
        metadata: HashMap::new(),
    };

    let file = File {
        file_id: "file-123".to_string(),
        name: "example.txt".to_string(),
        content: vec![72, 101, 108, 108, 111], // "Hello" in ASCII
        metadata: HashMap::new(),
    };

    upload_file(&mut directory, file, &mut files);
    update_file_content("file-123", vec![87, 111, 114, 108, 100], &mut files); // "World" in ASCII
}