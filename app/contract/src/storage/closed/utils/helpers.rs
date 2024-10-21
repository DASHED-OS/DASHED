use std::collections::HashMap;
use crate::storage::closed::models::access_control::{AccessControlList, AccessControlEntry};

// Function to log access attempts
pub fn log_access_attempt(user_id: &str, resource: &str, success: bool) {
    let status = if success { "successful" } else { "failed" };
    println!("Access attempt by user {} to resource {} was {}", user_id, resource, status);
}

// Function to check and log access
pub fn check_and_log_access(acl: &AccessControlList, user_id: &str, resource: &str, permission: &str) -> bool {
    let has_access = acl.entries.get(user_id)
        .map_or(false, |entry| entry.permissions.contains(&permission.to_string()));
    
    log_access_attempt(user_id, resource, has_access);
    has_access
}

// Function to list all users with a specific permission
pub fn list_users_with_permission(acl: &AccessControlList, permission: &str) -> Vec<String> {
    acl.entries.iter()
        .filter_map(|(user_id, entry)| {
            if entry.permissions.contains(&permission.to_string()) {
                Some(user_id.clone())
            } else {
                None
            }
        })
        .collect()
}

// Example usage
fn main() {
    let mut acl = AccessControlList {
        entries: HashMap::new(),
    };

    let entry = AccessControlEntry {
        user_id: "user123".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    acl.entries.insert(entry.user_id.clone(), entry);

    check_and_log_access(&acl, "user123", "file1", "read");
    let users_with_read = list_users_with_permission(&acl, "read");
    println!("Users with read permission: {:?}", users_with_read);
}