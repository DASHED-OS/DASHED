use crate::storage::closed::models::access_control::{AccessControlList, AccessControlEntry, create_access_control_entry, add_access_control_entry, has_permission};

// Function to grant permissions to a user
pub fn grant_permission(acl: &mut AccessControlList, user_id: &str, permission: &str) {
    if let Some(entry) = acl.entries.get_mut(user_id) {
        if !entry.permissions.contains(&permission.to_string()) {
            entry.permissions.push(permission.to_string());
        }
    } else {
        let entry = create_access_control_entry(user_id, vec![permission.to_string()]);
        add_access_control_entry(acl, entry);
    }
}

// Function to revoke permissions from a user
pub fn revoke_permission(acl: &mut AccessControlList, user_id: &str, permission: &str) {
    if let Some(entry) = acl.entries.get_mut(user_id) {
        entry.permissions.retain(|p| p != permission);
    }
}

// Function to check if a user has a specific permission
pub fn check_permission(acl: &AccessControlList, user_id: &str, permission: &str) -> bool {
    has_permission(acl, user_id, permission)
}

// Example usage
fn main() {
    let mut acl = AccessControlList {
        entries: std::collections::HashMap::new(),
    };

    grant_permission(&mut acl, "user123", "read");
    grant_permission(&mut acl, "user123", "write");

    println!("User has read permission: {}", check_permission(&acl, "user123", "read"));
    println!("User has write permission: {}", check_permission(&acl, "user123", "write"));

    revoke_permission(&mut acl, "user123", "write");
    println!("User has write permission after revocation: {}", check_permission(&acl, "user123", "write"));
}