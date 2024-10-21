use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};

// Define a struct to represent configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub settings: HashMap<String, String>,
}

// Function to create a new configuration
pub fn create_config() -> StorageConfig {
    StorageConfig {
        settings: HashMap::new(),
    }
}

// Function to set a configuration value
pub fn set_config_value(config: &mut StorageConfig, key: &str, value: &str) {
    config.settings.insert(key.to_string(), value.to_string());
}

// Function to get a configuration value
pub fn get_config_value(config: &StorageConfig, key: &str) -> Option<&String> {
    config.settings.get(key)
}

// Function to remove a configuration value
pub fn remove_config_value(config: &mut StorageConfig, key: &str) {
    config.settings.remove(key);
}

// Function to load a configuration from a JSON file
pub fn load_config<P: AsRef<Path>>(path: P) -> io::Result<StorageConfig> {
    let content = fs::read_to_string(path)?;
    let config = serde_json::from_str(&content)?;
    Ok(config)
}

// Function to save a configuration to a JSON file
pub fn save_config<P: AsRef<Path>>(path: P, config: &StorageConfig) -> io::Result<()> {
    let content = serde_json::to_string_pretty(config)?;