use std::collections::HashMap;
use crate::smart_resources::compose::models::service_definition::ServiceDefinition;

// Function to merge environment variables from two services
pub fn merge_environment_variables(
    service1: &ServiceDefinition,
    service2: &ServiceDefinition,
) -> HashMap<String, String> {
    let mut merged_env = service1.environment.clone();
    for (key, value) in &service2.environment {
        merged_env.insert(key.clone(), value.clone());
    }
    merged_env
}

// Function to validate a service definition
pub fn validate_service_definition(service: &ServiceDefinition) -> Result<(), String> {
    if service.name.is_empty() {
        return Err("Service name cannot be empty.".to_string());
    }
    if service.image.is_empty() {
        return Err("Service image cannot be empty.".to_string());
    }
    Ok(())
}

// Example usage
fn main() {
    let service1 = ServiceDefinition {
        name: "web".to_string(),
        image: "nginx:latest".to_string(),
        ports: vec!["80:80".to_string()],
        environment: HashMap::new(),
        volumes: vec!["/data".to_string()],
        networks: vec!["frontend".to_string()],
    };

    let service2 = ServiceDefinition {
        name: "api".to_string(),
        image: "node:14".to_string(),
        ports: vec!["3000:3000".to_string()],
        environment: [("NODE_ENV".to_string(), "production".to_string())]
            .iter()
            .cloned()
            .collect(),
        volumes: vec!["/app".to_string()],
        networks: vec!["backend".to_string()],
    };

    let merged_env = merge_environment_variables(&service1, &service2);
    println!("Merged Environment Variables: {:?}", merged_env);

    match validate_service_definition(&service1) {
        Ok(_) => println!("Service 1 is valid."),
        Err(e) => println!("Service 1 validation error: {}", e),
    }
}