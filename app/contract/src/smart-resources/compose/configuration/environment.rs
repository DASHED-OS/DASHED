use std::collections::HashMap;

// Define a struct to represent environment configurations
#[derive(Debug, Clone)]
pub struct EnvironmentConfig {
    pub variables: HashMap<String, String>,
}

// Function to create a new environment configuration
pub fn create_environment_config() -> EnvironmentConfig {
    EnvironmentConfig {
        variables: HashMap::new(),
    }
}

// Function to set an environment variable
pub fn set_environment_variable(config: &mut EnvironmentConfig, key: &str, value: &str) {
    config.variables.insert(key.to_string(), value.to_string());
}

// Function to get an environment variable
pub fn get_environment_variable(config: &EnvironmentConfig, key: &str) -> Option<&String> {
    config.variables.get(key)
}

// Example usage
fn main() {
    let mut config = create_environment_config();
    set_environment_variable(&mut config, "DATABASE_URL", "postgres://user:password@localhost/db");
    set_environment_variable(&mut config, "REDIS_URL", "redis://localhost:6379");

    if let Some(database_url) = get_environment_variable(&config, "DATABASE_URL") {
        println!("Database URL: {}", database_url);
    }

    if let Some(redis_url) = get_environment_variable(&config, "REDIS_URL") {
        println!("Redis URL: {}", redis_url);
    }
}