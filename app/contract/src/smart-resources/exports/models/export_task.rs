use std::collections::HashMap;
use std::time::SystemTime;

// Define a struct to represent an export task
#[derive(Debug, Clone)]
pub struct ExportTask {
    pub task_id: String,
    pub format: String,
    pub destination: String,
    pub status: String,
    pub created_at: SystemTime,
    pub details: HashMap<String, String>,
}

// Function to create a new export task
pub fn create_export_task(task_id: &str, format: &str, destination: &str) -> ExportTask {
    ExportTask {
        task_id: task_id.to_string(),
        format: format.to_string(),
        destination: destination.to_string(),
        status: "Pending".to_string(),
        created_at: SystemTime::now(),
        details: HashMap::new(),
    }
}

// Function to update the status of an export task
pub fn update_task_status(task: &mut ExportTask, status: &str) {
    task.status = status.to_string();
}

// Function to add a detail to an export task
pub fn add_task_detail(task: &mut ExportTask, key: &str, value: &str) {
    task.details.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut task = create_export_task("task-123", "CSV", "exports/services.csv");
    update_task_status(&mut task, "In Progress");
    add_task_detail(&mut task, "record_count", "100");
    println!("Export Task: {:?}", task);
}