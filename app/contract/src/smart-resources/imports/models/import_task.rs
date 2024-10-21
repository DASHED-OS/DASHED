use std::collections::HashMap;
use std::time::SystemTime;

// Define a struct to represent an import task
#[derive(Debug, Clone)]
pub struct ImportTask {
    pub task_id: String,
    pub format: String,
    pub source: String,
    pub status: String,
    pub created_at: SystemTime,
    pub details: HashMap<String, String>,
}

// Function to create a new import task
pub fn create_import_task(task_id: &str, format: &str, source: &str) -> ImportTask {
    ImportTask {
        task_id: task_id.to_string(),
        format: format.to_string(),
        source: source.to_string(),
        status: "Pending".to_string(),
        created_at: SystemTime::now(),
        details: HashMap::new(),
    }
}

// Function to update the status of an import task
pub fn update_task_status(task: &mut ImportTask, status: &str) {
    task.status = status.to_string();
}

// Function to add a detail to an import task
pub fn add_task_detail(task: &mut ImportTask, key: &str, value: &str) {
    task.details.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut task = create_import_task("task-123", "CSV", "imports/services.csv");
    update_task_status(&mut task, "In Progress");
    add_task_detail(&mut task, "record_count", "100");
    println!("Import Task: {:?}", task);
}