use std::time::SystemTime;
use std::collections::HashMap;

// Define a struct to represent storage metrics
#[derive(Debug, Clone)]
pub struct StorageMetrics {
    pub timestamp: SystemTime,
    pub total_capacity_gb: u64,
    pub used_capacity_gb: u64,
    pub free_capacity_gb: u64,
    pub volume_metrics: HashMap<String, VolumeMetrics>,
}

// Define a struct to represent metrics for a specific storage volume
#[derive(Debug, Clone)]
pub struct VolumeMetrics {
    pub capacity_gb: u64,
    pub used_gb: u64,
    pub free_gb: u64,
}

// Function to create a new storage metrics entry
pub fn create_storage_metrics(total_capacity_gb: u64, used_capacity_gb: u64, volume_metrics: HashMap<String, VolumeMetrics>) -> StorageMetrics {
    let free_capacity_gb = total_capacity_gb - used_capacity_gb;
    StorageMetrics {
        timestamp: SystemTime::now(),
        total_capacity_gb,
        used_capacity_gb,
        free_capacity_gb,
        volume_metrics,
    }
}

// Function to simulate collecting storage metrics
pub fn collect_storage_metrics() -> StorageMetrics {
    // Simulate storage usage data
    let total_capacity_gb = 1000; // Example value
    let used_capacity_gb = 600; // Example value
    let volume_metrics = vec![
        ("volume_1".to_string(), VolumeMetrics { capacity_gb: 500, used_gb: 300, free_gb: 200 }),
        ("volume_2".to_string(), VolumeMetrics { capacity_gb: 500, used_gb: 300, free_gb: 200 }),
    ].into_iter().collect();

    create_storage_metrics(total_capacity_gb, used_capacity_gb, volume_metrics)
}

// Example usage
fn main() {
    let metrics = collect_storage_metrics();
    println!("Storage Metrics: {:?}", metrics);
}