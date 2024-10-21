use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use serde_json;
use crate::smart_resources::compose::models::service_definition::ServiceDefinition;

// Function to export a list of service definitions to a JSON file
pub fn export_services_to_json<P: AsRef<Path>>(path: P, services: &[ServiceDefinition]) -> io::Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, &services)?;
    Ok(())
}

// Example usage
fn main() -> io::Result<()> {
    let services = vec![
        ServiceDefinition {
            name: "web".to_string(),
            image: "nginx:latest".to_string(),
            ports: vec!["80:80".to_string()],
            environment: [("ENV".to_string(), "production".to_string())].iter().cloned().collect(),
            volumes: vec!["/data".to_string()],
            networks: vec!["frontend".to_string()],
        },
    ];

    export_services_to_json("services.json", &services)?;
    println!("Services exported to JSON successfully.");

    Ok(())
}