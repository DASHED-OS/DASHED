use std::collections::HashMap;

// Define a struct to represent a Docker Compose network
#[derive(Debug, Clone)]
pub struct DockerComposeNetwork {
    pub network_name: String,
    pub driver: String,
    pub options: HashMap<String, String>,
}

// Function to create a new Docker Compose network
pub fn create_docker_compose_network(network_name: &str, driver: &str) -> DockerComposeNetwork {
    DockerComposeNetwork {
        network_name: network_name.to_string(),
        driver: driver.to_string(),
        options: HashMap::new(),
    }
}

// Function to add an option to a Docker Compose network
pub fn add_network_option(network: &mut DockerComposeNetwork, key: &str, value: &str) {
    network.options.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut network = create_docker_compose_network("my_network", "bridge");
    add_network_option(&mut network, "com.docker.network.bridge.name", "br0");

    println!("Docker Compose Network: {:?}", network);
}