use std::fs;
use std::io::{self, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};

// Function to load a configuration from a JSON file
pub fn load_config<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> io::Result<T> {
    let content = fs::read_to_string(path)?;
    let config = serde_json::from_str(&content)?;
    Ok(config)
}

// Function to save a configuration to a JSON file
pub fn save_config<P: AsRef<Path>, T: Serialize>(path: P, config: &T) -> io::Result<()> {
    let content = serde_json::to_string_pretty(config)?;
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// Example usage
fn main() -> io::Result<()> {
    #[derive(Serialize, Deserialize, Debug)]
    struct ExampleConfig {
        name: String,
        value: i32,
    }

    let config = ExampleConfig {
        name: "example".to_string(),
        value: 42,
    };

    save_config("config.json", &config)?;
    let loaded_config: ExampleConfig = load_config("config.json")?;
    println!("Loaded Config: {:?}", loaded_config);

    Ok(())
}