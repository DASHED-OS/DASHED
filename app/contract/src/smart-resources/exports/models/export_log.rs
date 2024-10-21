use std::collections::HashMap;
use std::time::SystemTime;

// Define a struct to represent an export log entry
#[derive(Debug, Clone)]
pub struct ExportLog {
    pub timestamp: SystemTime,
    pub format: String,
    pub file_path: String,
    pub status: String,
    pub details: HashMap<String, String>,
}

// Function to create a new export log entry
pub fn create_export_log(format: &str, file_path: &str, status: &str) -> ExportLog {
    ExportLog {
        timestamp: SystemTime::now(),
        format: format.to_string(),
        file_path: file_path.to_string(),
        status: status.to_string(),
        details: HashMap::new(),
    }
}

// Function to add a detail to an export log entry
pub fn add_log_detail(log: &mut ExportLog, key: &str, value: &str) {
    log.details.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut log = create_export_log("CSV", "exports/services.csv", "Success");
    add_log_detail(&mut log, "record_count", "100");
    println!("Export Log: {:?}", log);
}