use std::collections::HashMap;
use std::time::SystemTime;

// Define a struct to represent an alert
#[derive(Debug, Clone)]
pub struct Alert {
    pub timestamp: SystemTime,
    pub alert_type: String,
    pub message: String,
    pub details: HashMap<String, String>,
}

// Function to create a new alert
pub fn create_alert(alert_type: &str, message: &str) -> Alert {
    Alert {
        timestamp: SystemTime::now(),
        alert_type: alert_type.to_string(),
        message: message.to_string(),
        details: HashMap::new(),
    }
}

// Function to add a detail to an alert
pub fn add_alert_detail(alert: &mut Alert, key: &str, value: &str) {
    alert.details.insert(key.to_string(), value.to_string());
}

// Function to trigger an alert based on a threshold
pub fn trigger_alert_if_exceeds_threshold(metric_name: &str, value: f64, threshold: f64) -> Option<Alert> {
    if value > threshold {
        let mut alert = create_alert("Threshold Exceeded", &format!("{} exceeded threshold with value: {}", metric_name, value));
        add_alert_detail(&mut alert, "metric_name", metric_name);
        add_alert_detail(&mut alert, "threshold", &threshold.to_string());
        add_alert_detail(&mut alert, "value", &value.to_string());
        Some(alert)
    } else {
        None
    }
}