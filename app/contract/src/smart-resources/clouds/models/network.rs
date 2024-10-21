use std::collections::HashMap;

// Define a struct to represent a network configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub network_id: String,
    pub cidr_block: String,
    pub status: String,
    pub tags: HashMap<String, String>,
}

// Function to create a new network configuration
pub fn create_network(network_id: &str, cidr_block: &str) -> NetworkConfig {
    NetworkConfig {
        network_id: network_id.to_string(),
        cidr_block: cidr_block.to_string(),
        status: "active".to_string(),
        tags: HashMap::new(),
    }
}

// Function to update the status of a network configuration
pub fn update_network_status(network: &mut NetworkConfig, new_status: &str) {
    network.status = new_status.to_string();
}

// Function to add a tag to a network configuration
pub fn add_network_tag(network: &mut NetworkConfig, key: &str, value: &str) {
    network.tags.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut network = create_network("net-12345678", "192.168.0.0/16");
    println!("Created Network: {:?}", network);

    update_network_status(&mut network, "inactive");
    println!("Updated Network Status: {:?}", network);

    add_network_tag(&mut network, "environment", "production");
    println!("Updated Network Tags: {:?}", network);
}