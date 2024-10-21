use std::time::{SystemTime, Duration};
use std::collections::HashMap;

// Define a struct to represent CPU metrics
#[derive(Debug, Clone)]
pub struct CpuMetrics {
    pub timestamp: SystemTime,
    pub usage_percent: f32,
    pub core_usages: HashMap<String, f32>,
}

// Function to create a new CPU metrics entry
pub fn create_cpu_metrics(usage_percent: f32, core_usages: HashMap<String, f32>) -> CpuMetrics {
    CpuMetrics {
        timestamp: SystemTime::now(),
        usage_percent,
        core_usages,
    }
}

// Function to simulate collecting CPU metrics
pub fn collect_cpu_metrics() -> CpuMetrics {
    // Simulate CPU usage data
    let usage_percent = 45.0; // Example value
    let core_usages = vec![
        ("core_0".to_string(), 40.0),
        ("core_1".to_string(), 50.0),
    ].into_iter().collect();

    create_cpu_metrics(usage_percent, core_usages)
}

// Example usage
fn main() {
    let metrics = collect_cpu_metrics();
    println!("CPU Metrics: {:?}", metrics);
}