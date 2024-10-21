use std::collections::HashMap;
use std::time::SystemTime;

// Define a struct to represent a log entry
#[derive(Debug, Clone)]
pub struct MonitorLog {
    pub timestamp: SystemTime,
    pub log_type: String,
    pub message: String,
    pub details: HashMap<String, String>,
}

// Function to create a new log entry
pub fn create_log(log_type: &str, message: &str) -> MonitorLog {
    MonitorLog {
        timestamp: SystemTime::now(),
        log_type: log_type.to_string(),
        message: message.to_string(),
        details: HashMap::new(),
    }
}

// Function to add a detail to a log entry
pub fn add_log_detail(log: &mut MonitorLog, key: &str, value: &str) {
    log.details.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut log = create_log("Info", "Monitoring started");
    add_log_detail(&mut log, "component", "CPU Monitor");
    println!("Monitor Log: {:?}", log);
}