use std::collections::HashMap;
use crate::storage::closed::models::access_control::{AccessControlList, AccessControlEntry};

// Define a struct to represent a secure directory
#[derive(Debug, Clone)]
pub struct SecureDirectory {
    pub directory_id: String,
    pub name: String,
    pub access_control: AccessControlList,
    pub metadata: HashMap<String, String>,
}

// Function to create a new secure directory
pub fn create_secure_directory(directory_id: &str, name: &str) -> SecureDirectory {
    SecureDirectory {
        directory_id: directory_id.to_string(),
        name: name.to_string(),
        access_control: AccessControlList {
            entries: HashMap::new(),
        },
        metadata: HashMap::new(),
    }
}

// Function to add an access control entry to a secure directory
pub fn add_access_control_to_directory(directory: &mut SecureDirectory, entry: AccessControlEntry) {
    directory.access_control.entries.insert(entry.user_id.clone(), entry);
}

// Function to add metadata to a secure directory
pub fn add_metadata_to_directory(directory: &mut SecureDirectory, key: &str, value: &str) {
    directory.metadata.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut directory = create_secure_directory("dir-12345678", "SecureDocs");
    println!("Created Secure Directory: {:?}", directory);

    let entry = AccessControlEntry {
        user_id: "user123".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    add_access_control_to_directory(&mut directory, entry);

    add_metadata_to_directory(&mut directory, "owner", "admin");
    println!("Updated Secure Directory: {:?}", directory);
}