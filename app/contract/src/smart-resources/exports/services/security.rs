use crate::smart_resources::exports::models::{export_task::ExportTask, export_policy::ExportPolicy};

// Function to validate an export task against a security policy
pub fn validate_export_task(task: &ExportTask, policy: &ExportPolicy) -> Result<(), String> {
    if !policy.formats_allowed.contains(&task.format) {
        return Err(format!("Format '{}' is not allowed by the policy.", task.format));
    }

    if let Some(max_size) = policy.additional_settings.get("max_file_size_mb") {
        let max_size: u32 = max_size.parse().unwrap_or(0);
        if max_size > 0 && task.details.get("file_size_mb").map_or(0, |s| s.parse().unwrap_or(0)) > max_size {
            return Err(format!("File size exceeds the maximum allowed size of {} MB.", max_size));
        }
    }

    Ok(())
}

// Example usage
fn main() {
    let policy = ExportPolicy {
        policy_name: "Default Policy".to_string(),
        formats_allowed: vec!["CSV".to_string(), "JSON".to_string()],
        max_file_size_mb: 100,
        additional_settings: [("max_file_size_mb".to_string(), "100".to_string())].iter().cloned().collect(),
    };

    let task = ExportTask {
        task_id: "task-123".to_string(),
        format: "CSV".to_string(),
        destination: "exports/services.csv".to_string(),
        status: "Pending".to_string(),
        created_at: std::time::SystemTime::now(),
        details: [("file_size_mb".to_string(), "50".to_string())].iter().cloned().collect(),
    };

    match validate_export_task(&task, &policy) {
        Ok(_) => println!("Export task is valid."),
        Err(e) => println!("Export task validation failed: {}", e),
    }
}