use std::collections::HashMap;
use crate::storage::open::models::file::File;
use crate::storage::open::models::directory::Directory;

// Function to add a file to a directory
pub fn add_file_to_directory(directory: &mut Directory, file: File) {
    directory.files.push(file.file_id.clone());
    println!("Added file {} to directory {}", file.name, directory.name);
}

// Function to remove a file from a directory
pub fn remove_file_from_directory(directory: &mut Directory, file_id: &str) -> bool {
    if let Some(pos) = directory.files.iter().position(|id| id == file_id) {
        directory.files.remove(pos);
        println!("Removed file {} from directory {}", file_id, directory.name);
        true
    } else {
        println!("File {} not found in directory {}", file_id, directory.name);
        false
    }
}

// Function to list all files in a directory
pub fn list_files_in_directory(directory: &Directory, files: &HashMap<String, File>) {
    println!("Files in directory {}:", directory.name);
    for file_id in &directory.files {
        if let Some(file) = files.get(file_id) {
            println!(" - {}", file.name);
        }
    }
}

// Example usage
fn main() {
    let mut files = HashMap::new();
    let file1 = File {
        file_id: "file-123".to_string(),
        name: "example1.txt".to_string(),
        content: vec![],
        metadata: HashMap::new(),
    };
    let file2 = File {
        file_id: "file-456".to_string(),
        name: "example2.txt".to_string(),
        content: vec![],
        metadata: HashMap::new(),
    };
    files.insert(file1.file_id.clone(), file1.clone());
    files.insert(file2.file_id.clone(), file2.clone());

    let mut directory = Directory {
        directory_id: "dir-123".to_string(),
        name: "Documents".to_string(),
        files: vec![],
        metadata: HashMap::new(),
    };

    add_file_to_directory(&mut directory, file1);
    add_file_to_directory(&mut directory, file2);
    list_files_in_directory(&directory, &files);
    remove_file_from_directory(&mut directory, "file-123");
    list_files_in_directory(&directory, &files);
}