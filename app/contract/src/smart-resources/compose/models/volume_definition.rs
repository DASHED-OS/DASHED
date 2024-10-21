use std::collections::HashMap;

// Define a struct to represent a Docker Compose volume definition
#[derive(Debug, Clone)]
pub struct VolumeDefinition {
    pub name: String,
    pub driver: String,
    pub options: HashMap<String, String>,
}

// Function to create a new volume definition
pub fn create_volume_definition(name: &str, driver: &str) -> VolumeDefinition {
    VolumeDefinition {
        name: name.to_string(),
        driver: driver.to_string(),
        options: HashMap::new(),
    }
}

// Function to add an option to a volume definition
pub fn add_volume_option(volume: &mut VolumeDefinition, key: &str, value: &str) {
    volume.options.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut volume = create_volume_definition("my_volume", "local");
    add_volume_option(&mut volume, "device", "/dev/sda1");
    add_volume_option(&mut volume, "o", "bind");

    println!("Volume Definition: {:?}", volume);
}