use std::time::SystemTime;
use std::collections::HashMap;

// Define a generic struct to represent a metric
#[derive(Debug, Clone)]
pub struct Metric<T> {
    pub timestamp: SystemTime,
    pub data: T,
    pub tags: HashMap<String, String>,
}

// Function to create a new metric
pub fn create_metric<T>(data: T, tags: HashMap<String, String>) -> Metric<T> {
    Metric {
        timestamp: SystemTime::now(),
        data,
        tags,
    }
}

// Example usage
fn main() {
    let cpu_usage = 45.0; // Example CPU usage percentage
    let mut tags = HashMap::new();
    tags.insert("host".to_string(), "server1".to_string());

    let cpu_metric = create_metric(cpu_usage, tags);
    println!("CPU Metric: {:?}", cpu_metric);
}