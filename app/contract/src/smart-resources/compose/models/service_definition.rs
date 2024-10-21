use std::collections::HashMap;

// Define a struct to represent a Docker Compose service definition
#[derive(Debug, Clone)]
pub struct ServiceDefinition {
    pub name: String,
    pub image: String,
    pub ports: Vec<String>,
    pub environment: HashMap<String, String>,
    pub volumes: Vec<String>,
    pub networks: Vec<String>,
}

// Function to create a new service definition
pub fn create_service_definition(name: &str, image: &str) -> ServiceDefinition {
    ServiceDefinition {
        name: name.to_string(),
        image: image.to_string(),
        ports: Vec::new(),
        environment: HashMap::new(),
        volumes: Vec::new(),
        networks: Vec::new(),
    }
}

// Function to add a port to a service definition
pub fn add_service_port(service: &mut ServiceDefinition, port: &str) {
    service.ports.push(port.to_string());
}

// Function to add an environment variable to a service definition
pub fn add_service_environment_variable(service: &mut ServiceDefinition, key: &str, value: &str) {
    service.environment.insert(key.to_string(), value.to_string());
}

// Function to add a volume to a service definition
pub fn add_service_volume(service: &mut ServiceDefinition, volume: &str) {
    service.volumes.push(volume.to_string());
}

// Function to add a network to a service definition
pub fn add_service_network(service: &mut ServiceDefinition, network: &str) {
    service.networks.push(network.to_string());
}

// Example usage
fn main() {
    let mut service = create_service_definition("web", "nginx:latest");
    add_service_port(&mut service, "80:80");
    add_service_environment_variable(&mut service, "ENV", "production");
    add_service_volume(&mut service, "/data");
    add_service_network(&mut service, "frontend");

    println!("Service Definition: {:?}", service);
}