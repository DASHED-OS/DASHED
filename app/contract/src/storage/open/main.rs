use std::collections::HashMap;
use crate::storage::open::models::file::File;
use crate::storage::open::models::directory::Directory;
use crate::storage::open::services::upload::upload_file;
use crate::storage::open::services::download::download_file;
use crate::storage::open::services::management::{add_file_to_directory, list_files_in_directory};
use crate::storage::open::utils::helpers::calculate_total_size;

fn main() {
    // Initialize storage for files and directories
    let mut files = HashMap::new();
    let mut directory = Directory {
        directory_id: "dir-123".to_string(),
        name: "Documents".to_string(),
        files: vec![],
        metadata: HashMap::new(),
    };

    // Create a new file
    let file = File {
        file_id: "file-123".to_string(),
        name: "example.txt".to_string(),
        content: vec![72, 101, 108, 108, 111], // "Hello" in ASCII
        metadata: HashMap::new(),
    };

    // Upload the file to the directory
    upload_file(&mut directory, file.clone(), &mut files);

    // List files in the directory
    list_files_in_directory(&directory, &files);

    // Calculate total size of files in the directory
    let total_size = calculate_total_size(&directory, &files);
    println!("Total size of files in directory: {} bytes", total_size);

    // Download the file
    if let Some(downloaded_file) = download_file("file-123", &files) {
        println!("Downloaded File: {:?}", downloaded_file);
    } else {
        println!("File not found.");
    }
}