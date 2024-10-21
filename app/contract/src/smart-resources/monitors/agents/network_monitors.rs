use std::collections::HashMap;
use crate::smart_resources::metrics::collections::network::{NetworkMetrics, collect_network_metrics};
use crate::smart_resources::metrics::services::alerting::{trigger_alert_if_exceeds_threshold, Alert};

// Function to monitor network usage and trigger alerts if necessary
pub fn monitor_network_usage(threshold: f64) -> Option<Alert> {
    let network_metrics = collect_network_metrics();
    println!("Monitoring Network Metrics: {:?}", network_metrics);

    if network_metrics.usage_percent > threshold {
        trigger_alert_if_exceeds_threshold("Network Usage", network_metrics.usage_percent, threshold)
    } else {
        None
    }
}

// Example usage
fn main() {
    let network_threshold = 75.0; // Example threshold

    if let Some(alert) = monitor_network_usage(network_threshold) {
        println!("Alert Triggered: {:?}", alert);
    } else {
        println!("Network usage is within acceptable limits.");
    }
}