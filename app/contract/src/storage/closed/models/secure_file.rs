use std::collections::HashMap;
use crate::storage::closed::models::access_control::{AccessControlList, AccessControlEntry};

// Define a struct to represent a secure file
#[derive(Debug, Clone)]
pub struct SecureFile {
    pub file_id: String,
    pub name: String,
    pub content: Vec<u8>,
    pub access_control: AccessControlList,
    pub metadata: HashMap<String, String>,
}

// Function to create a new secure file
pub fn create_secure_file(file_id: &str, name: &str, content: Vec<u8>) -> SecureFile {
    SecureFile {
        file_id: file_id.to_string(),
        name: name.to_string(),
        content,
        access_control: AccessControlList {
            entries: HashMap::new(),
        },
        metadata: HashMap::new(),
    }
}

// Function to add an access control entry to a secure file
pub fn add_access_control_to_file(file: &mut SecureFile, entry: AccessControlEntry) {
    file.access_control.entries.insert(entry.user_id.clone(), entry);
}

// Function to add metadata to a secure file
pub fn add_metadata_to_file(file: &mut SecureFile, key: &str, value: &str) {
    file.metadata.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut file = create_secure_file("file-12345678", "SecretDoc", vec![1, 2, 3, 4, 5]);
    println!("Created Secure File: {:?}", file);

    let entry = AccessControlEntry {
        user_id: "user123".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    add_access_control_to_file(&mut file, entry);

    add_metadata_to_file(&mut file, "owner", "admin");
    println!("Updated Secure File: {:?}", file);
}