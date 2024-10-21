use std::collections::HashMap;
use std::time::SystemTime;
use crate::smart_resources::metrics::collections::cpu::{CpuMetrics, collect_cpu_metrics};
use crate::smart_resources::metrics::services::alerting::{trigger_alert_if_exceeds_threshold, Alert};

// Function to monitor CPU usage and trigger alerts if necessary
pub fn monitor_cpu_usage(threshold: f32) -> Option<Alert> {
    let cpu_metrics = collect_cpu_metrics();
    println!("Monitoring CPU Metrics: {:?}", cpu_metrics);

    if cpu_metrics.usage_percent > threshold {
        trigger_alert_if_exceeds_threshold("CPU Usage", cpu_metrics.usage_percent as f64, threshold as f64)
    } else {
        None
    }
}

// Example usage
fn main() {
    let cpu_threshold = 80.0; // Example threshold

    if let Some(alert) = monitor_cpu_usage(cpu_threshold) {
        println!("Alert Triggered: {:?}", alert);
    } else {
        println!("CPU usage is within acceptable limits.");
    }
}