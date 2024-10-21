use std::collections::HashMap;

// Define a struct to represent the configuration for a monitor
#[derive(Debug, Clone)]
pub struct MonitorConfig {
    pub monitor_type: String,
    pub threshold: f64,
    pub settings: HashMap<String, String>,
}

// Function to create a new monitor configuration
pub fn create_monitor_config(monitor_type: &str, threshold: f64) -> MonitorConfig {
    MonitorConfig {
        monitor_type: monitor_type.to_string(),
        threshold,
        settings: HashMap::new(),
    }
}

// Function to add a setting to a monitor configuration
pub fn add_monitor_setting(config: &mut MonitorConfig, key: &str, value: &str) {
    config.settings.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut config = create_monitor_config("CPU Monitor", 80.0);
    add_monitor_setting(&mut config, "interval", "5s");
    println!("Monitor Config: {:?}", config);
}