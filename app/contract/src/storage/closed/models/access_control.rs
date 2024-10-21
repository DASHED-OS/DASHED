use std::collections::HashMap;

// Define a struct to represent an access control entry
#[derive(Debug, Clone)]
pub struct AccessControlEntry {
    pub user_id: String,
    pub permissions: Vec<String>,
}

// Define a struct to represent an access control list
#[derive(Debug, Clone)]
pub struct AccessControlList {
    pub entries: HashMap<String, AccessControlEntry>,
}

// Function to create a new access control entry
pub fn create_access_control_entry(user_id: &str, permissions: Vec<String>) -> AccessControlEntry {
    AccessControlEntry {
        user_id: user_id.to_string(),
        permissions,
    }
}

// Function to add an entry to an access control list
pub fn add_access_control_entry(acl: &mut AccessControlList, entry: AccessControlEntry) {
    acl.entries.insert(entry.user_id.clone(), entry);
}

// Function to check if a user has a specific permission
pub fn has_permission(acl: &AccessControlList, user_id: &str, permission: &str) -> bool {
    if let Some(entry) = acl.entries.get(user_id) {
        entry.permissions.contains(&permission.to_string())
    } else {
        false
    }
}

// Example usage
fn main() {
    let mut acl = AccessControlList {
        entries: HashMap::new(),
    };

    let entry = create_access_control_entry("user123", vec!["read".to_string(), "write".to_string()]);
    add_access_control_entry(&mut acl, entry);

    let has_read_permission = has_permission(&acl, "user123", "read");
    println!("User has read permission: {}", has_read_permission);
}