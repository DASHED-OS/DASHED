use std::collections::HashMap;
use crate::storage::open::models::file::File;
use crate::storage::open::models::metadata::Metadata;

// Function to download a file by its ID
pub fn download_file(file_id: &str, files: &HashMap<String, File>) -> Option<File> {
    files.get(file_id).cloned()
}

// Function to get metadata for a file
pub fn get_file_metadata(file_id: &str, metadata_store: &HashMap<String, Metadata>) -> Option<Metadata> {
    metadata_store.get(file_id).cloned()
}

// Example usage
fn main() {
    let mut files = HashMap::new();
    let file = File {
        file_id: "file-123".to_string(),
        name: "example.txt".to_string(),
        content: vec![72, 101, 108, 108, 111], // "Hello" in ASCII
        metadata: HashMap::new(),
    };
    files.insert(file.file_id.clone(), file);

    let mut metadata_store = HashMap::new();
    let metadata = Metadata {
        item_id: "file-123".to_string(),
        attributes: HashMap::new(),
    };
    metadata_store.insert(metadata.item_id.clone(), metadata);

    if let Some(downloaded_file) = download_file("file-123", &files) {
        println!("Downloaded File: {:?}", downloaded_file);
    } else {
        println!("File not found.");
    }

    if let Some(file_metadata) = get_file_metadata("file-123", &metadata_store) {
        println!("File Metadata: {:?}", file_metadata);
    } else {
        println!("Metadata not found.");
    }
}