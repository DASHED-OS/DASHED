use std::collections::HashMap;

// Define a struct to represent a Docker Compose service
#[derive(Debug, Clone)]
pub struct DockerComposeService {
    pub service_name: String,
    pub image: String,
    pub ports: Vec<String>,
    pub environment: HashMap<String, String>,
}

// Function to create a new Docker Compose service
pub fn create_docker_compose_service(service_name: &str, image: &str) -> DockerComposeService {
    DockerComposeService {
        service_name: service_name.to_string(),
        image: image.to_string(),
        ports: Vec::new(),
        environment: HashMap::new(),
    }
}

// Function to add a port to a Docker Compose service
pub fn add_port(service: &mut DockerComposeService, port: &str) {
    service.ports.push(port.to_string());
}

// Function to add an environment variable to a Docker Compose service
pub fn add_environment_variable(service: &mut DockerComposeService, key: &str, value: &str) {
    service.environment.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut service = create_docker_compose_service("web", "nginx:latest");
    add_port(&mut service, "80:80");
    add_environment_variable(&mut service, "ENV", "production");

    println!("Docker Compose Service: {:?}", service);
}