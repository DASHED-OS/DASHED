use std::collections::HashMap;
use crate::smart_resources::monitors::models::alert::Alert;
use crate::smart_resources::metrics::collections::cpu::CpuMetrics;
use crate::smart_resources::metrics::collections::memory::MemoryMetrics;
use crate::smart_resources::metrics::collections::network::NetworkMetrics;
use crate::smart_resources::metrics::collections::storage::StorageMetrics;

// Function to display CPU metrics on the dashboard
pub fn display_cpu_metrics(cpu_metrics: &CpuMetrics) {
    println!("CPU Metrics:");
    println!("Usage Percent: {:.2}%", cpu_metrics.usage_percent);
    for (core, usage) in &cpu_metrics.core_usages {
        println!("Core {}: {:.2}%", core, usage);
    }
}

// Function to display memory metrics on the dashboard
pub fn display_memory_metrics(memory_metrics: &MemoryMetrics) {
    println!("Memory Metrics:");
    println!("Total Memory: {} MB", memory_metrics.total_memory_mb);
    println!("Used Memory: {} MB", memory_metrics.used_memory_mb);
    println!("Free Memory: {} MB", memory_metrics.free_memory_mb);
}

// Function to display network metrics on the dashboard
pub fn display_network_metrics(network_metrics: &NetworkMetrics) {
    println!("Network Metrics:");
    println!("Usage Percent: {:.2}%", network_metrics.usage_percent);
}

// Function to display storage metrics on the dashboard
pub fn display_storage_metrics(storage_metrics: &StorageMetrics) {
    println!("Storage Metrics:");
    println!("Used Capacity: {} GB", storage_metrics.used_capacity_gb);
}

// Function to display alerts on the dashboard
pub fn display_alerts(alerts: &[Alert]) {
    println!("Alerts:");
    for alert in alerts {
        println!("Alert Type: {}", alert.alert_type);
        println!("Message: {}", alert.message);
        for (key, value) in &alert.details {
            println!("{}: {}", key, value);
        }
    }
}

// Example usage
fn main() {
    // Example data
    let cpu_metrics = CpuMetrics {
        timestamp: std::time::SystemTime::now(),
        usage_percent: 75.0,
        core_usages: HashMap::new(),
    };
    let memory_metrics = MemoryMetrics {
        timestamp: std::time::SystemTime::now(),
        total_memory_mb: 8192,
        used_memory_mb: 4096,
        free_memory_mb: 4096,
        process_memory_usage: HashMap::new(),
    };
    let network_metrics = NetworkMetrics {
        timestamp: std::time::SystemTime::now(),
        usage_percent: 60.0,
    };
    let storage_metrics = StorageMetrics {
        timestamp: std::time::SystemTime::now(),
        used_capacity_gb: 300,
    };
    let alerts = vec![
        Alert {
            timestamp: std::time::SystemTime::now(),
            alert_type: "High CPU Usage".to_string(),
            message: "CPU usage exceeded 80%".to_string(),
            details: HashMap::new(),
        },
    ];

    // Display metrics and alerts
    display_cpu_metrics(&cpu_metrics);
    display_memory_metrics(&memory_metrics);
    display_network_metrics(&network_metrics);
    display_storage_metrics(&storage_metrics);
    display_alerts(&alerts);
}