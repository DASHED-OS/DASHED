mod imports {
    pub mod services {
        pub mod audit;
        pub mod import_manager;
        pub mod security;
    }
    pub mod formats {
        pub mod csv;
        pub mod json;
    }
    pub mod models {
        pub mod import_log;
        pub mod import_task;
        pub mod import_policy;
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

use imports::services::audit::*;
use imports::services::import_manager::*;
use imports::services::security::*;
use imports::formats::csv::*;
use imports::formats::json::*;
use imports::models::import_log::*;
use imports::models::import_task::*;
use imports::models::import_policy::*;
use imports::utils::config::*;
use imports::utils::helper::*;
use compose::utils::config::*;
use compose::models::service_definition::*;

fn main() -> io::Result<()> {
    // Create an import policy
    let policy = create_import_policy("Default Import Policy", vec!["CSV".to_string(), "JSON".to_string()], 100);

    // Create an import task
    let mut task = create_import_task("task-123", "CSV", "imports/services.csv");

    // Validate the import task
    match validate_import_task(&task, &policy) {
        Ok(_) => println!("Import task is valid."),
        Err(e) => {
            println!("Import task validation failed: {}", e);
            return Ok(());
        }
    }

    // Execute the import
    let manager = ImportManager::new(policy);
    let services = manager.execute_import(&mut task)?;
    let log = manager.log_import(&task);
    println!("Import Log: {:?}", log);

    // Log the import action
    log_import_action("import", &task.source);

    Ok(())
}