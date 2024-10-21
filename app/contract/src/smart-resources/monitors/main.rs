use crate::smart_resources::monitors::agents::cpu_monitors::monitor_cpu_usage;
use crate::smart_resources::monitors::agents::memory_monitor::monitor_memory_usage;
use crate::smart_resources::monitors::agents::network_monitors::monitor_network_usage;
use crate::smart_resources::monitors::agents::storage_monitor::monitor_storage_usage;
use crate::smart_resources::monitors::models::monitor_config::MonitorConfig;
use crate::smart_resources::monitors::services::alerting::display_alerts;
use crate::smart_resources::monitors::services::dashboard::display_metrics;
use crate::smart_resources::monitors::services::logging::log_event;
use std::collections::HashMap;

fn main() {
    // Example configurations
    let cpu_config = MonitorConfig {
        monitor_type: "CPU".to_string(),
        threshold: 80.0,
        settings: HashMap::new(),
    };

    let memory_config = MonitorConfig {
        monitor_type: "Memory".to_string(),
        threshold: 70.0,
        settings: HashMap::new(),
    };

    let network_config = MonitorConfig {
        monitor_type: "Network".to_string(),
        threshold: 75.0,
        settings: HashMap::new(),
    };

    let storage_config = MonitorConfig {
        monitor_type: "Storage".to_string(),
        threshold: 400.0,
        settings: HashMap::new(),
    };

    // Monitor CPU usage
    if let Some(alert) = monitor_cpu_usage(cpu_config.threshold as f32) {
        display_alerts(&[alert]);
    }

    // Monitor Memory usage
    if let Some(alert) = monitor_memory_usage(memory_config.threshold as f32) {
        display_alerts(&[alert]);
    }

    // Monitor Network usage
    if let Some(alert) = monitor_network_usage(network_config.threshold) {
        display_alerts(&[alert]);
    }

    // Monitor Storage usage
    if let Some(alert) = monitor_storage_usage(storage_config.threshold as u64) {
        display_alerts(&[alert]);
    }

    // Log the monitoring event
    let mut event_details = HashMap::new();
    event_details.insert("event".to_string(), "Monitoring cycle completed".to_string());
    log_event("Info", "Monitoring cycle completed", event_details);

    // Display metrics on the dashboard
    display_metrics();
}