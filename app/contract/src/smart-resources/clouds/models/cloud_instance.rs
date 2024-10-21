use std::collections::HashMap;

// Define a struct to represent a cloud instance
#[derive(Debug, Clone)]
pub struct CloudInstance {
    pub instance_id: String,
    pub instance_type: String,
    pub status: String,
    pub metadata: HashMap<String, String>,
}

// Function to create a new cloud instance
pub fn create_instance(instance_id: &str, instance_type: &str) -> CloudInstance {
    CloudInstance {
        instance_id: instance_id.to_string(),
        instance_type: instance_type.to_string(),
        status: "running".to_string(),
        metadata: HashMap::new(),
    }
}

// Function to update the status of a cloud instance
pub fn update_instance_status(instance: &mut CloudInstance, new_status: &str) {
    instance.status = new_status.to_string();
}

// Function to add metadata to a cloud instance
pub fn add_instance_metadata(instance: &mut CloudInstance, key: &str, value: &str) {
    instance.metadata.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut instance = create_instance("i-1234567890abcdef0", "t2.micro");
    println!("Created Instance: {:?}", instance);

    update_instance_status(&mut instance, "stopped");
    println!("Updated Instance Status: {:?}", instance);

    add_instance_metadata(&mut instance, "owner", "admin");
    println!("Updated Instance Metadata: {:?}", instance);
}