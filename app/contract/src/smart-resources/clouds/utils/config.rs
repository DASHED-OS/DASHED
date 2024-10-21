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

// Function to remove a configuration value
pub fn remove_config_value(config: &mut Config, key: &str) {
    config.settings.remove(key);
}

// Example usage
fn main() {
    let mut config = create_config();
    set_config_value(&mut config, "region", "us-west-2");
    set_config_value(&mut config, "instance_type", "t2.micro");

    if let Some(region) = get_config_value(&config, "region") {
        println!("Region: {}", region);
    }

    remove_config_value(&mut config, "instance_type");
    println!("Config after removal: {:?}", config);
}