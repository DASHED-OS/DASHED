use std::collections::HashMap;

// Define a struct to represent configuration settings
#[derive(Debug, Clone)]
pub struct Config {
    pub settings: HashMap<String, String>,
}

// Function to create a new configuration
pub fn create_config() -> Config {
    Config {
        settings: HashMap::new(),
    }
}

// Function to set a configuration value
pub fn set_config_value(config: &mut Config, key: &str, value: &str) {
    config.settings.insert(key.to_string(), value.to_string());
}

// Function to get a configuration value
pub fn get_config_value(config: &Config, key: &str) -> Option<&String> {
    config.settings.get(key)
}

// Example usage
fn main() {
    let mut config = create_config();
    set_config_value(&mut config, "max_replicas", "10");
    set_config_value(&mut config, "min_replicas", "1");

    if let Some(max_replicas) = get_config_value(&config, "max_replicas") {
        println!("Max Replicas: {}", max_replicas);
    }

    if let Some(min_replicas) = get_config_value(&config, "min_replicas") {
        println!("Min Replicas: {}", min_replicas);
    }
}