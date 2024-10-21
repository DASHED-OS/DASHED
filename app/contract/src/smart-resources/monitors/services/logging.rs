use std::collections::HashMap;
use std::time::SystemTime;
use crate::smart_resources::monitors::models::log::{MonitorLog, create_log, add_log_detail};

// Function to log a monitoring event
pub fn log_event(log_type: &str, message: &str, details: HashMap<String, String>) {
    let mut log = create_log(log_type, message);
    for (key, value) in details {
        add_log_detail(&mut log, &key, &value);
    }
    println!("Log Event: {:?}", log);
}

// Function to log an alert
pub fn log_alert(alert_type: &str, message: &str, details: HashMap<String, String>) {
    let mut log = create_log(alert_type, message);
    for (key, value) in details {
        add_log_detail(&mut log, &key, &value);
    }
    println!("Alert Log: {:?}", log);
}

// Example usage
fn main() {
    let mut event_details = HashMap::new();
    event_details.insert("component".to_string(), "CPU Monitor".to_string());
    log_event("Info", "Monitoring started", event_details);

    let mut alert_details = HashMap::new();
    alert_details.insert("threshold".to_string(), "80%".to_string());
    alert_details.insert("current_usage".to_string(), "85%".to_string());
    log_alert("Warning", "CPU usage exceeded threshold", alert_details);
}