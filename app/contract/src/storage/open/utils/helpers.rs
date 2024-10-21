use std::collections::HashMap;
use crate::storage::open::models::file::File;
use crate::storage::open::models::directory::Directory;

// Function to calculate the total size of files in a directory
pub fn calculate_total_size(directory: &Directory, files: &HashMap<String, File>) -> usize {
    directory.files.iter()
        .filter_map(|file_id| files.get(file_id))
        .map(|file| file.content.len())
        .sum()
}

// Function to print details of a directory
pub fn print_directory_details(directory: &Directory, files: &HashMap<String, File>) {
    println!("Directory ID: {}", directory.directory_id);
    println!("Directory Name: {}", directory.name);
    println!("Files in Directory:");
    for file_id in &directory.files {
        if let Some(file) = files.get(file_id) {
            println!(" - {} ({} bytes)", file.name, file.content.len());
        }
    }
}

// Example usage
fn main() {
    let mut files = HashMap::new();
    let file1 = File {
        file_id: "file-123".to_string(),
        name: "example1.txt".to_string(),
        content: vec![0; 100], // 100 bytes
        metadata: HashMap::new(),
    };
    let file2 = File {
        file_id: "file-456".to_string(),
        name: "example2.txt".to_string(),
        content: vec![0; 200], // 200 bytes
        metadata: HashMap::new(),
    };
    files.insert(file1.file_id.clone(), file1);
    files.insert(file2.file_id.clone(), file2);

    let directory = Directory {
        directory_id: "dir-123".to_string(),
        name: "Documents".to_string(),
        files: vec!["file-123".to_string(), "file-456".to_string()],
        metadata: HashMap::new(),
    };

    print_directory_details(&directory, &files);
    let total_size = calculate_total_size(&directory, &files);
    println!("Total size of files in directory: {} bytes", total_size);
}