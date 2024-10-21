use crate::smart_resources::exports::models::{export_task::ExportTask, export_log::ExportLog, export_policy::ExportPolicy};
use crate::smart_resources::exports::formats::{csv::export_services_to_csv, json::export_services_to_json};
use crate::smart_resources::compose::models::service_definition::ServiceDefinition;
use std::io;

pub struct ExportManager {
    pub policy: ExportPolicy,
}

impl ExportManager {
    pub fn new(policy: ExportPolicy) -> Self {
        ExportManager { policy }
    }

    pub fn execute_export(&self, task: &mut ExportTask, services: &[ServiceDefinition]) -> io::Result<()> {
        if !self.policy.formats_allowed.contains(&task.format) {
            task.status = "Failed".to_string();
            return Err(io::Error::new(io::ErrorKind::Other, "Format not allowed by policy"));
        }

        match task.format.as_str() {
            "CSV" => export_services_to_csv(&task.destination, services)?,
            "JSON" => export_services_to_json(&task.destination, services)?,
            _ => {
                task.status = "Failed".to_string();
                return Err(io::Error::new(io::ErrorKind::Other, "Unsupported format"));
            }
        }

        task.status = "Completed".to_string();
        Ok(())
    }

    pub fn log_export(&self, task: &ExportTask) -> ExportLog {
        let mut log = ExportLog {
            timestamp: task.created_at,
            format: task.format.clone(),
            file_path: task.destination.clone(),
            status: task.status.clone(),
            details: task.details.clone(),
        };
        log
    }
}

// Example usage
fn main() -> io::Result<()> {
    let policy = ExportPolicy {
        policy_name: "Default Policy".to_string(),
        formats_allowed: vec!["CSV".to_string(), "JSON".to_string()],
        max_file_size_mb: 100,
        additional_settings: std::collections::HashMap::new(),
    };

    let mut task = ExportTask {
        task_id: "task-123".to_string(),
        format: "CSV".to_string(),
        destination: "exports/services.csv".to_string(),
        status: "Pending".to_string(),
        created_at: std::time::SystemTime::now(),
        details: std::collections::HashMap::new(),
    };

    let services = vec![
        ServiceDefinition {
            name: "web".to_string(),
            image: "nginx:latest".to_string(),
            ports: vec!["80:80".to_string()],
            environment: [("ENV".to_string(), "production".to_string())].iter().cloned().collect(),
            volumes: vec!["/data".to_string()],
            networks: vec!["frontend".to_string()],
        },
    ];

    let manager = ExportManager::new(policy);
    manager.execute_export(&mut task, &services)?;
    let log = manager.log_export(&task);
    println!("Export Log: {:?}", log);

    Ok(())
}