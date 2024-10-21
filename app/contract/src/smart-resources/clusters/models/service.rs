use std::collections::HashMap;

// Define a struct to represent a service in a cluster
#[derive(Debug, Clone)]
pub struct Service {
    pub service_id: String,
    pub service_name: String,
    pub status: String,
    pub labels: HashMap<String, String>,
}

// Function to create a new service
pub fn create_service(service_id: &str, service_name: &str) -> Service {
    Service {
        service_id: service_id.to_string(),
        service_name: service_name.to_string(),
        status: "active".to_string(),
        labels: HashMap::new(),
    }
}

// Function to update the status of a service
pub fn update_service_status(service: &mut Service, new_status: &str) {
    service.status = new_status.to_string();
}

// Function to add a label to a service
pub fn add_service_label(service: &mut Service, key: &str, value: &str) {
    service.labels.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut service = create_service("s-1234567890abcdef0", "web-service");
    println!("Created Service: {:?}", service);

    update_service_status(&mut service, "inactive");
    println!("Updated Service Status: {:?}", service);

    add_service_label(&mut service, "version", "v1.0");
    println!("Updated Service Labels: {:?}", service);
}