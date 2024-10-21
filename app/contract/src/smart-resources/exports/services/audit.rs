use std::collections::HashMap;
use std::time::SystemTime;
use crate::smart_resources::compose::models::service_definition::ServiceDefinition;

// Define a struct to represent an audit log entry
#[derive(Debug, Clone)]
pub struct AuditLog {
    pub timestamp: SystemTime,
    pub action: String,
    pub service_name: String,
    pub details: HashMap<String, String>,
}

// Function to create a new audit log entry
pub fn create_audit_log(action: &str, service: &ServiceDefinition) -> AuditLog {
    AuditLog {
        timestamp: SystemTime::now(),
        action: action.to_string(),
        service_name: service.name.clone(),
        details: HashMap::new(),
    }
}

// Function to add a detail to an audit log entry
pub fn add_audit_detail(log: &mut AuditLog, key: &str, value: &str) {
    log.details.insert(key.to_string(), value.to_string());
}

// Function to log an action performed on a service
pub fn log_service_action(action: &str, service: &ServiceDefinition) {
    let mut log = create_audit_log(action, service);
    add_audit_detail(&mut log, "image", &service.image);
    println!("Audit Log: {:?}", log);
}

// Example usage
fn main() {
    let service = ServiceDefinition {
        name: "web".to_string(),
        image: "nginx:latest".to_string(),
        ports: vec!["80:80".to_string()],
        environment: [("ENV".to_string(), "production".to_string())].iter().cloned().collect(),
        volumes: vec!["/data".to_string()],
        networks: vec!["frontend".to_string()],
    };

    log_service_action("start", &service);
}