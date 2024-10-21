use std::collections::HashMap;

// Define a struct to represent an Azure instance
#[derive(Debug, Clone)]
pub struct AzureInstance {
    pub instance_id: String,
    pub instance_type: String,
    pub region: String,
    pub status: String,
    pub tags: HashMap<String, String>,
}

// Function to create a new Azure instance
pub fn create_azure_instance(instance_id: &str, instance_type: &str, region: &str) -> AzureInstance {
    AzureInstance {
        instance_id: instance_id.to_string(),
        instance_type: instance_type.to_string(),
        region: region.to_string(),
        status: "running".to_string(),
        tags: HashMap::new(),
    }
}

// Function to update the status of an Azure instance
pub fn update_azure_instance_status(instance: &mut AzureInstance, new_status: &str) {
    instance.status = new_status.to_string();
}

// Function to add a tag to an Azure instance
pub fn add_azure_instance_tag(instance: &mut AzureInstance, key: &str, value: &str) {
    instance.tags.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut instance = create_azure_instance("az-1234567890abcdef0", "Standard_B1s", "eastus");
    println!("Created Azure Instance: {:?}", instance);

    update_azure_instance_status(&mut instance, "stopped");
    println!("Updated Azure Instance Status: {:?}", instance);

    add_azure_instance_tag(&mut instance, "environment", "production");
    println!("Updated Azure Instance Tags: {:?}", instance);
}