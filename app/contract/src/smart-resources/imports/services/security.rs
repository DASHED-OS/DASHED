use crate::smart_resources::imports::models::import_policy::ImportPolicy;
use crate::smart_resources::imports::models::import_task::ImportTask;
use std::collections::HashMap;
use std::io;

// Define a struct to represent security settings for imports
#[derive(Debug, Clone)]
pub struct ImportSecuritySettings {
    pub allowed_sources: Vec<String>,
    pub max_file_size_mb: u32,
    pub additional_checks: HashMap<String, String>,
}

impl ImportSecuritySettings {
    pub fn new(allowed_sources: Vec<String>, max_file_size_mb: u32) -> Self {
        ImportSecuritySettings {
            allowed_sources,
            max_file_size_mb,
            additional_checks: HashMap::new(),
        }
    }

    pub fn add_check(&mut self, key: &str, value: &str) {
        self.additional_checks.insert(key.to_string(), value.to_string());
    }
}

// Function to validate an import task against security settings
pub fn validate_import_task(task: &ImportTask, security_settings: &ImportSecuritySettings) -> io::Result<()> {
    if !security_settings.allowed_sources.contains(&task.source) {
        return Err(io::Error::new(io::ErrorKind::Other, "Source not allowed by security settings"));
    }

    if task.details.get("file_size_mb").map_or(0, |size| size.parse::<u32>().unwrap_or(0)) > security_settings.max_file_size_mb {
        return Err(io::Error::new(io::ErrorKind::Other, "File size exceeds security limits"));
    }

    // Additional checks can be added here
    Ok(())
}

// Example usage
fn main() -> io::Result<()> {
    let mut security_settings = ImportSecuritySettings::new(vec!["trusted_source.csv".to_string()], 100);
    security_settings.add_check("encryption", "AES256");

    let task = ImportTask {
        task_id: "task-123".to_string(),
        format: "CSV".to_string(),
        source: "trusted_source.csv".to_string(),
        status: "Pending".to_string(),
        created_at: std::time::SystemTime::now(),
        details: [("file_size_mb".to_string(), "50".to_string())].iter().cloned().collect(),
    };

    validate_import_task(&task, &security_settings)?;
    println!("Import task is valid and secure.");

    Ok(())
}