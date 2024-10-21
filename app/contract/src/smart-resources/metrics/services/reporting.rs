use std::collections::HashMap;
use std::time::SystemTime;

// Define a struct to represent a report
#[derive(Debug, Clone)]
pub struct Report {
    pub timestamp: SystemTime,
    pub report_type: String,
    pub details: HashMap<String, String>,
}

// Function to create a new report
pub fn create_report(report_type: &str, details: HashMap<String, String>) -> Report {
    Report {
        timestamp: SystemTime::now(),
        report_type: report_type.to_string(),
        details,
    }
}

// Function to add a detail to a report
pub fn add_report_detail(report: &mut Report, key: &str, value: &str) {
    report.details.insert(key.to_string(), value.to_string());
}

// Function to generate a report from metrics
pub fn generate_metrics_report(metrics: &HashMap<String, f64>) -> Report {
    let mut details = HashMap::new();
    for (metric_name, value) in metrics {
        details.insert(metric_name.clone(), format!("{}%", value));
    }
    create_report("Metrics Report", details)
}

// Example usage
fn main() {
    let mut metrics = HashMap::new();
    metrics.insert("CPU Usage".to_string(), 45.0);
    metrics.insert("Memory Usage".to_string(), 50.0);

    let report = generate_metrics_report(&metrics);
    println!("Generated Report: {:?}", report);
}