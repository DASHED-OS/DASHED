use std::collections::HashMap;
use crate::smart_resources::metrics::collections::storage::{StorageMetrics, collect_storage_metrics};
use crate::smart_resources::metrics::services::alerting::{trigger_alert_if_exceeds_threshold, Alert};

// Function to monitor storage usage and trigger alerts if necessary
pub fn monitor_storage_usage(threshold: u64) -> Option<Alert> {
    let storage_metrics = collect_storage_metrics();
    println!("Monitoring Storage Metrics: {:?}", storage_metrics);

    if storage_metrics.used_capacity_gb > threshold {
        trigger_alert_if_exceeds_threshold("Storage Usage", storage_metrics.used_capacity_gb as f64, threshold as f64)
    } else {
        None
    }
}

// Example usage
fn main() {
    let storage_threshold = 400; // Example threshold in GB

    if let Some(alert) = monitor_storage_usage(storage_threshold) {
        println!("Alert Triggered: {:?}", alert);
    } else {
        println!("Storage usage is within acceptable limits.");
    }
}