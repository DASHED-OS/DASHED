use crate::storage::closed::models::access_control::{AccessControlList, create_access_control_entry, add_access_control_entry, has_permission};
use crate::storage::closed::models::secure_file::SecureFile;
use crate::storage::closed::services::encryption::encrypt_file;
use crate::storage::closed::services::decryption::decrypt_file;
use crate::storage::closed::utils::helpers::{log_access_attempt, check_and_log_access};
use std::collections::HashMap;

fn main() {
    // Initialize access control list
    let mut acl = AccessControlList {
        entries: HashMap::new(),
    };

    // Create and add access control entry
    let entry = create_access_control_entry("user123", vec!["read".to_string(), "write".to_string()]);
    add_access_control_entry(&mut acl, entry);

    // Check and log access
    let has_access = check_and_log_access(&acl, "user123", "file1", "read");
    println!("User has access: {}", has_access);

    // Create a secure file
    let mut file = SecureFile {
        file_id: "file-12345678".to_string(),
        name: "SecretDoc".to_string(),
        content: vec![1, 2, 3, 4, 5], // Original content
        access_control: acl.clone(),
        metadata: HashMap::new(),
    };

    // Encrypt the file
    match encrypt_file(&mut file, "correct_key") {
        Ok(_) => println!("File encrypted successfully: {:?}", file.content),
        Err(err) => println!("Encryption failed: {}", err),
    }

    // Decrypt the file
    match decrypt_file(&file, "correct_key") {
        Ok(decrypted_content) => println!("Decrypted content: {:?}", decrypted_content),
        Err(err) => println!("Decryption failed: {}", err),
    }
}