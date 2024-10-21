use std::collections::HashMap;

// Define a struct to represent a container
#[derive(Debug, Clone)]
pub struct Container {
    pub container_id: String,
    pub image: String,
    pub status: String,
    pub environment_vars: HashMap<String, String>,
}

// Function to create a new container
pub fn create_container(container_id: &str, image: &str) -> Container {
    Container {
        container_id: container_id.to_string(),
        image: image.to_string(),
        status: "created".to_string(),
        environment_vars: HashMap::new(),
    }
}

// Function to update the status of a container
pub fn update_container_status(container: &mut Container, new_status: &str) {
    container.status = new_status.to_string();
}

// Function to add an environment variable to a container
pub fn add_container_env_var(container: &mut Container, key: &str, value: &str) {
    container.environment_vars.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut container = create_container("c-1234567890abcdef0", "nginx:latest");
    println!("Created Container: {:?}", container);

    update_container_status(&mut container, "running");
    println!("Updated Container Status: {:?}", container);

    add_container_env_var(&mut container, "ENV", "production");
    println!("Updated Container Environment Variables: {:?}", container);
}