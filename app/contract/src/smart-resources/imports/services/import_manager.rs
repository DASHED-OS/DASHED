use crate::smart_resources::imports::models::{import_task::ImportTask, import_log::ImportLog, import_policy::ImportPolicy};
use crate::smart_resources::imports::formats::{csv::import_services_from_csv, json::import_services_from_json};
use crate::smart_resources::compose::models::service_definition::ServiceDefinition;
use std::io;

pub struct ImportManager {
    pub policy: ImportPolicy,
}

impl ImportManager {
    pub fn new(policy: ImportPolicy) -> Self {
        ImportManager { policy }
    }

    pub fn execute_import(&self, task: &mut ImportTask) -> io::Result<Vec<ServiceDefinition>> {
        if !self.policy.formats_allowed.contains(&task.format) {
            task.status = "Failed".to_string();
            return Err(io::Error::new(io::ErrorKind::Other, "Format not allowed by policy"));
        }

        let services = match task.format.as_str() {
            "CSV" => import_services_from_csv(&task.source)?,
            "JSON" => import_services_from_json(&task.source)?,
            _ => {
                task.status = "Failed".to_string();
                return Err(io::Error::new(io::ErrorKind::Other, "Unsupported format"));
            }
        };

        task.status = "Completed".to_string();
        Ok(services)
    }

    pub fn log_import(&self, task: &ImportTask) -> ImportLog {
        let mut log = ImportLog {
            timestamp: task.created_at,
            format: task.format.clone(),
            file_path: task.source.clone(),
            status: task.status.clone(),
            details: task.details.clone(),
        };
        log
    }
}

// Example usage
fn main() -> io::Result<()> {
    let policy = ImportPolicy {
        policy_name: "Default Import Policy".to_string(),
        formats_allowed: vec!["CSV".to_string(), "JSON".to_string()],
        max_file_size_mb: 100,
        additional_settings: std::collections::HashMap::new(),
    };

    let mut task = ImportTask {
        task_id: "task-123".to_string(),
        format: "CSV".to_string(),
        source: "imports/services.csv".to_string(),
        status: "Pending".to_string(),
        created_at: std::time::SystemTime::now(),
        details: std::collections::HashMap::new(),
    };

    let manager = ImportManager::new(policy);
    let services = manager.execute_import(&mut task)?;
    let log = manager.log_import(&task);
    println!("Import Log: {:?}", log);

    Ok(())
}