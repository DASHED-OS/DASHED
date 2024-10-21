use std::collections::HashMap;

// Define a struct to represent a Docker Compose volume
#[derive(Debug, Clone)]
pub struct DockerComposeVolume {
    pub volume_name: String,
    pub driver: String,
    pub options: HashMap<String, String>,
}

// Function to create a new Docker Compose volume
pub fn create_docker_compose_volume(volume_name: &str, driver: &str) -> DockerComposeVolume {
    DockerComposeVolume {
        volume_name: volume_name.to_string(),
        driver: driver.to_string(),
        options: HashMap::new(),
    }
}

// Function to add an option to a Docker Compose volume
pub fn add_volume_option(volume: &mut DockerComposeVolume, key: &str, value: &str) {
    volume.options.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut volume = create_docker_compose_volume("my_volume", "local");
    add_volume_option(&mut volume, "device", "/dev/sda1");
    add_volume_option(&mut volume, "o", "bind");

    println!("Docker Compose Volume: {:?}", volume);
}