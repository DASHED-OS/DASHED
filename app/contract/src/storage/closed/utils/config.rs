use std::collections::HashMap;

// Define a struct to represent configuration settings for the storage system
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub encryption_enabled: bool,
    pub max_storage_capacity_gb: u32,
    pub access_control: HashMap<String, Vec<String>>, // Maps user IDs to their permissions
}

// Function to create a new storage configuration
pub fn create_storage_config(encryption_enabled: bool, max_storage_capacity_gb: u32) -> StorageConfig {
    StorageConfig {
        encryption_enabled,
        max_storage_capacity_gb,
        access_control: HashMap::new(),
    }
}

// Function to add a user with permissions to the storage configuration
pub fn add_user_permission(config: &mut StorageConfig, user_id: &str, permissions: Vec<String>) {
    config.access_control.insert(user_id.to_string(), permissions);
}

// Function to check if a user has a specific permission
pub fn user_has_permission(config: &StorageConfig, user_id: &str, permission: &str) -> bool {
    if let Some(user_permissions) = config.access_control.get(user_id) {
        user_permissions.contains(&permission.to_string())
    } else {
        false
    }
}

// Example usage
fn main() {
    let mut config = create_storage_config(true, 1000);
    add_user_permission(&mut config, "user123", vec!["read".to_string(), "write".to_string()]);

    let has_read_permission = user_has_permission(&config, "user123", "read");
    println!("User has read permission: {}", has_read_permission);
}