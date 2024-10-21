use std::collections::HashMap;

// Function to check if a specific configuration is supported by an OS
pub fn is_configuration_supported(
    os_configurations: &HashMap<String, String>, 
    config_key: &str, 
    config_value: &str
) -> bool {
    match os_configurations.get(config_key) {
        Some(value) => value == config_value,
        None => false,
    }
}

// Function to merge two OS configuration maps
pub fn merge_os_configurations(
    base_config: &mut HashMap<String, String>, 
    additional_config: HashMap<String, String>
) {
    for (key, value) in additional_config {
        base_config.insert(key, value);
    }
}

// Function to list all configuration keys for a given OS
pub fn list_configuration_keys(os_configurations: &HashMap<String, String>) -> Vec<String> {
    os_configurations.keys().cloned().collect()
}

// Example usage
fn main() {
    let mut linux_config = HashMap::from([
        ("Kernel".to_string(), "5.10".to_string()),
        ("Shell".to_string(), "bash".to_string()),
    ]);

    let additional_config = HashMap::from([
        ("PackageManager".to_string(), "apt".to_string()),
    ]);

    merge_os_configurations(&mut linux_config, additional_config);

    let is_supported = is_configuration_supported(&linux_config, "Kernel", "5.10");
    println!("Is Kernel 5.10 supported? {}", is_supported);

    let config_keys = list_configuration_keys(&linux_config);
    println!("Configuration keys: {:?}", config_keys);
}