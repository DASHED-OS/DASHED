use std::collections::HashMap;
use std::time::SystemTime;

// Define a struct to represent an import log entry
#[derive(Debug, Clone)]
pub struct ImportLog {
    pub timestamp: SystemTime,
    pub format: String,
    pub file_path: String,
    pub status: String,
    pub details: HashMap<String, String>,
}

// Function to create a new import log entry
pub fn create_import_log(format: &str, file_path: &str, status: &str) -> ImportLog {
    ImportLog {
        timestamp: SystemTime::now(),
        format: format.to_string(),
        file_path: file_path.to_string(),
        status: status.to_string(),
        details: HashMap::new(),
    }
}

// Function to add a detail to an import log entry
pub fn add_log_detail(log: &mut ImportLog, key: &str, value: &str) {
    log.details.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut log = create_import_log("CSV", "imports/services.csv", "Success");
    add_log_detail(&mut log, "record_count", "100");
    println!("Import Log: {:?}", log);
}