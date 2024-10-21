use std::collections::HashMap;

// Define a struct to represent a GCP instance
#[derive(Debug, Clone)]
pub struct GcpInstance {
    pub instance_id: String,
    pub instance_type: String,
    pub zone: String,
    pub status: String,
    pub labels: HashMap<String, String>,
}

// Function to create a new GCP instance
pub fn create_gcp_instance(instance_id: &str, instance_type: &str, zone: &str) -> GcpInstance {
    GcpInstance {
        instance_id: instance_id.to_string(),
        instance_type: instance_type.to_string(),
        zone: zone.to_string(),
        status: "running".to_string(),
        labels: HashMap::new(),
    }
}

// Function to update the status of a GCP instance
pub fn update_gcp_instance_status(instance: &mut GcpInstance, new_status: &str) {
    instance.status = new_status.to_string();
}

// Function to add a label to a GCP instance
pub fn add_gcp_instance_label(instance: &mut GcpInstance, key: &str, value: &str) {
    instance.labels.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut instance = create_gcp_instance("gcp-1234567890abcdef0", "n1-standard-1", "us-central1-a");
    println!("Created GCP Instance: {:?}", instance);

    update_gcp_instance_status(&mut instance, "terminated");
    println!("Updated GCP Instance Status: {:?}", instance);

    add_gcp_instance_label(&mut instance, "environment", "development");
    println!("Updated GCP Instance Labels: {:?}", instance);
}