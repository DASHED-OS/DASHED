use std::collections::HashMap;

// Define a struct to represent a storage configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub storage_id: String,
    pub storage_type: String,
    pub capacity_gb: u32,
    pub status: String,
    pub tags: HashMap<String, String>,
}

// Function to create a new storage configuration
pub fn create_storage(storage_id: &str, storage_type: &str, capacity_gb: u32) -> StorageConfig {
    StorageConfig {
        storage_id: storage_id.to_string(),
        storage_type: storage_type.to_string(),
        capacity_gb,
        status: "available".to_string(),
        tags: HashMap::new(),
    }
}

// Function to update the status of a storage configuration
pub fn update_storage_status(storage: &mut StorageConfig, new_status: &str) {
    storage.status = new_status.to_string();
}

// Function to add a tag to a storage configuration
pub fn add_storage_tag(storage: &mut StorageConfig, key: &str, value: &str) {
    storage.tags.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut storage = create_storage("st-12345678", "SSD", 500);
    println!("Created Storage: {:?}", storage);

    update_storage_status(&mut storage, "in-use");
    println!("Updated Storage Status: {:?}", storage);

    add_storage_tag(&mut storage, "environment", "production");
    println!("Updated Storage Tags: {:?}", storage);
}