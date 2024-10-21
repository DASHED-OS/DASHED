use std::collections::HashMap;

// Define a struct to represent a threshold for a specific metric
#[derive(Debug, Clone)]
pub struct MetricThreshold {
    pub metric_name: String,
    pub upper_limit: f64,
    pub lower_limit: f64,
    pub tags: HashMap<String, String>,
}

// Function to create a new metric threshold
pub fn create_metric_threshold(metric_name: &str, upper_limit: f64, lower_limit: f64) -> MetricThreshold {
    MetricThreshold {
        metric_name: metric_name.to_string(),
        upper_limit,
        lower_limit,
        tags: HashMap::new(),
    }
}

// Function to add a tag to a metric threshold
pub fn add_threshold_tag(threshold: &mut MetricThreshold, key: &str, value: &str) {
    threshold.tags.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut cpu_threshold = create_metric_threshold("CPU Usage", 80.0, 20.0);
    add_threshold_tag(&mut cpu_threshold, "severity", "high");

    println!("Metric Threshold: {:?}", cpu_threshold);
}