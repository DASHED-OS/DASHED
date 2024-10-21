use std::time::SystemTime;
use std::collections::HashMap;

// Define a struct to represent network metrics
#[derive(Debug, Clone)]
pub struct NetworkMetrics {
    pub timestamp: SystemTime,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub interface_metrics: HashMap<String, InterfaceMetrics>,
}

// Define a struct to represent metrics for a specific network interface
#[derive(Debug, Clone)]
pub struct InterfaceMetrics {
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

// Function to create a new network metrics entry
pub fn create_network_metrics(total_bytes_sent: u64, total_bytes_received: u64, interface_metrics: HashMap<String, InterfaceMetrics>) -> NetworkMetrics {
    NetworkMetrics {
        timestamp: SystemTime::now(),
        total_bytes_sent,
        total_bytes_received,
        interface_metrics,
    }
}

// Function to simulate collecting network metrics
pub fn collect_network_metrics() -> NetworkMetrics {
    // Simulate network usage data
    let total_bytes_sent = 1024 * 1024; // Example value
    let total_bytes_received = 2048 * 1024; // Example value
    let interface_metrics = vec![
        ("eth0".to_string(), InterfaceMetrics { bytes_sent: 512 * 1024, bytes_received: 1024 * 1024 }),
        ("wlan0".to_string(), InterfaceMetrics { bytes_sent: 512 * 1024, bytes_received: 1024 * 1024 }),
    ].into_iter().collect();

    create_network_metrics(total_bytes_sent, total_bytes_received, interface_metrics)
}

// Example usage
fn main() {
    let metrics = collect_network_metrics();
    println!("Network Metrics: {:?}", metrics);
}