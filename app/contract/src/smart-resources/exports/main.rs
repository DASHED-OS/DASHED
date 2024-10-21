mod exports {
    pub mod services {
        pub mod audit;
        pub mod export_manager;
        pub mod security;
    }
    pub mod formats {
        pub mod csv;
        pub mod json;
    }
    pub mod models {
        pub mod export_log;
        pub mod export_task;
        pub mod export_policy;
    }
    pub mod utils {
        pub mod config;
        pub mod helper;
    }
}

mod compose {
    pub mod utils {
        pub mod config;
        pub mod helpers;
    }
    pub mod models {
        pub mod service_definition;
    }
}

use exports::services::audit::*;
use exports::services::export_manager::*;
use exports::services::security::*;
use exports::formats::csv::*;
use exports::formats::json::*;
use exports::models::export_log::*;
use exports::models::export_task::*;
use exports::models::export_policy::*;
use exports::utils::config::*;
use exports::utils::helper::*;
use compose::utils::config::*;
use compose::models::service_definition::*;

fn main() {
    // Create a service definition
    let service = ServiceDefinition {
        name: "web".to_string(),
        image: "nginx:latest".to_string(),
        ports: vec!["80:80".to_string()],
        environment: [("ENV".to_string(), "production".to_string())].iter().cloned().collect(),
        volumes: vec!["/data".to_string()],
        networks: vec!["frontend".to_string()],
    };

    // Log an action
    log_service_action("start", &service);

    // Create an export task
    let mut task = create_export_task("task-123", "CSV", "exports/services.csv");
    add_task_detail(&mut task, "file_size_mb", "50");

    // Create an export policy
    let policy = create_export_policy("Default Policy", vec!["CSV".to_string(), "JSON".to_string()], 100);

    // Validate the export task
    match validate_export_task(&task, &policy) {
        Ok(_) => println!("Export task is valid."),
        Err(e) => println!("Export task validation failed: {}", e),
    }

    // Execute the export
    let manager = ExportManager::new(policy);
    if let Err(e) = manager.execute_export(&mut task, &[service.clone()]) {
        println!("Failed to execute export: {}", e);
    }

    // Log the export
    let log = manager