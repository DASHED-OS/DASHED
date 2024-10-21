use std::fs;
use std::io;
use std::path::Path;
use serde::de::DeserializeOwned;

// Function to load a configuration from a JSON file
pub fn load_config<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> io::Result<T> {
    let content = fs::read_to_string(path)?;
    let config = serde_json::from_str(&content)?;
    Ok(config)
}

// Function to save a configuration to a JSON file
pub fn save_config<P: AsRef<Path>, T: serde::Serialize>(path: P, config: &T) -> io::Result<()> {
    let content = serde_json::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}

// Example usage
fn main() -> io::Result<()> {
    #[derive(serde::Deserialize, serde::Serialize, Debug)]
    struct ImportSettings {
        max_file_size_mb: u32,
        allowed_formats: Vec<String>,
    }

    let settings = ImportSettings {
        max_file_size_mb: 100,
        allowed_formats: vec!["CSV".to_string(), "JSON".to_string()],
    };

    save_config("import_settings.json", &settings)?;

    let loaded_settings: ImportSettings = load_config("import_settings.json")?;
    println!("Loaded Import Settings: {:?}", loaded_settings);

    Ok(())
}