use std::fs;
use std::io::{self, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::smart_resources::compose::models::service_definition::ServiceDefinition;

// Function to load a configuration from a JSON file
pub fn load_config<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> io::Result<T> {
    let content = fs::read_to_string(path)?;
    let config = serde_json::from_str(&content)?;
    Ok(config)
}

// Function to save a configuration to a JSON file
pub fn save_config<P: AsRef<Path>, T: Serialize>(path: P, config: &T) -> io::Result<()> {
    let content = serde_json::to_string_pretty(config)?;
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// Example usage
fn main() -> io::Result<()> {
    let service = ServiceDefinition {
        name: "web".to_string(),
        image: "nginx:latest".to_string(),
        ports: vec!["80:80".to_string()],
        environment: [("ENV".to_string(), "production".to_string())].iter().cloned().collect(),
        volumes: vec!["/data".to_string()],
        networks: vec!["frontend".to_string()],
    };

    save_config("service_config.json", &service)?;
    let loaded_service: ServiceDefinition = load_config("service_config.json")?;
    println!("Loaded Service: {:?}", loaded_service);

    Ok(())
}