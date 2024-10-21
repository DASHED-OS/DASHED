use crate::smart_resources::clusters::models::service::Service;
use crate::smart_resources::clusters::models::container::Container;
use crate::smart_resources::clusters::models::node::Node;

// Function to scale a service in a cluster
pub fn scale_service(service: &mut Service, new_status: &str) {
    println!("Scaling service: {:?}", service);
    service.status = new_status.to_string();
    // Logic to scale the service
}

// Function to scale a container in a cluster
pub fn scale_container(container: &mut Container, new_status: &str) {
    println!("Scaling container: {:?}", container);
    container.status = new_status.to_string();
    // Logic to scale the container
}

// Function to scale a node in a cluster
pub fn scale_node(node: &mut Node, new_status: &str) {
    println!("Scaling node: {:?}", node);
    node.status = new_status.to_string();
    // Logic to scale the node
}

// Example usage
fn main() {
    let mut service = Service {
        service_id: "s-1234567890abcdef0".to_string(),
        service_name: "web-service".to_string(),
        status: "active".to_string(),
        labels: std::collections::HashMap::new(),
    };
    scale_service(&mut service, "scaled");

    let mut container = Container {
        container_id: "c-1234567890abcdef0".to_string(),
        image: "nginx:latest".to_string(),
        status: "running".to_string(),
        environment_vars: std::collections::HashMap::new(),
    };
    scale_container(&mut container, "scaled");

    let mut node = Node {
        node_id: "n-1234567890abcdef0".to_string(),
        node_type: "compute".to_string(),
        status: "active".to_string(),
        labels: std::collections::HashMap::new(),
    };
    scale_node(&mut node, "scaled");
}