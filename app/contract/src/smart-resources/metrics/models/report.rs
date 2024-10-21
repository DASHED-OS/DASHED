use std::collections::HashMap;
use std::time::SystemTime;

// Define a struct to represent a generic metric report
#[derive(Debug, Clone)]
pub struct MetricReport {
    pub timestamp: SystemTime,
    pub report_type: String,
    pub details: HashMap<String, String>,
}

// Function to create a new metric report
pub fn create_metric_report(report_type: &str, details: HashMap<String, String>) -> MetricReport {
    MetricReport {
        timestamp: SystemTime::now(),
        report_type: report_type.to_string(),
        details,
    }
}

// Function to add a detail to a metric report
pub fn add_report_detail(report: &mut MetricReport, key: &str, value: &str) {
    report.details.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut details = HashMap::new();
    details.insert("CPU Usage".to_string(), "45%".to_string());
    details.insert("Memory Usage".to_string(), "50%".to_string());

    let mut report = create_metric_report("System Metrics", details);
    add_report_detail(&mut report, "Storage Usage", "60%");

    println!("Metric Report: {:?}", report);
}