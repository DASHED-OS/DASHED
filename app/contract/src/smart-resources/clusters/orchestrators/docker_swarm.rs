use std::collections::HashMap;

// Define a struct to represent a Docker Swarm service
#[derive(Debug, Clone)]
pub struct DockerSwarmService {
    pub service_id: String,
    pub service_name: String,
    pub replicas: u32,
    pub labels: HashMap<String, String>,
}

// Function to create a new Docker Swarm service
pub fn create_docker_swarm_service(service_id: &str, service_name: &str, replicas: u32) -> DockerSwarmService {
    DockerSwarmService {
        service_id: service_id.to_string(),
        service_name: service_name.to_string(),
        replicas,
        labels: HashMap::new(),
    }
}

// Function to scale a Docker Swarm service
pub fn scale_docker_swarm_service(service: &mut DockerSwarmService, new_replicas: u32) {
    println!("Scaling Docker Swarm service: {:?}", service);
    service.replicas = new_replicas;
    // Logic to scale the service
}

// Function to add a label to a Docker Swarm service
pub fn add_docker_swarm_service_label(service: &mut DockerSwarmService, key: &str, value: &str) {
    service.labels.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut service = create_docker_swarm_service("ds-1234567890abcdef0", "web-service", 3);
    println!("Created Docker Swarm Service: {:?}", service);

    scale_docker_swarm_service(&mut service, 5);
    println!("Scaled Docker Swarm Service: {:?}", service);

    add_docker_swarm_service_label(&mut service, "version", "v1.0");
    println!("Updated Docker Swarm Service Labels: {:?}", service);
}