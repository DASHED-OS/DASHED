use std::collections::HashMap;
use std::time::SystemTime;

// Define a struct to represent an analysis result
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub timestamp: SystemTime,
    pub metric_name: String,
    pub insights: HashMap<String, String>,
}

// Function to create a new analysis result
pub fn create_analysis_result(metric_name: &str) -> AnalysisResult {
    AnalysisResult {
        timestamp: SystemTime::now(),
        metric_name: metric_name.to_string(),
        insights: HashMap::new(),
    }
}

// Function to add an insight to an analysis result
pub fn add_insight(result: &mut AnalysisResult, key: &str, value: &str) {
    result.insights.insert(key.to_string(), value.to_string());
}

// Function to analyze CPU metrics and generate insights
pub fn analyze_cpu_metrics(cpu_usage: f32, core_usages: &HashMap<String, f32>) -> AnalysisResult {
    let mut result = create_analysis_result