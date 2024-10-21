use std::collections::HashMap;

// Define a struct to represent an OS mapping
#[derive(Debug, Clone)]
pub struct OSMapping {
    pub os_name: String,
    pub configurations: HashMap<String, String>,
}

// Function to add a new OS mapping
pub fn add_os_mapping(
    mappings: &mut HashMap<String, OSMapping>, 
    os_name: &str, 
    configurations: HashMap<String, String>
) {
    let mapping = OSMapping {
        os_name: os_name.to_string(),
        configurations,
    };
    mappings.insert(os_name.to_string(), mapping);
}

// Function to get configurations of a specific OS mapping
pub fn get_os_configurations(
    mappings: &HashMap<String, OSMapping>, 
    os_name: &str
) -> Option<HashMap<String, String>> {
    mappings.get(os_name).map(|mapping| mapping.configurations.clone())
}

// Function to list all OS mappings
pub fn list_os_mappings(mappings: &HashMap<String, OSMapping>) -> Vec<String> {
    mappings.keys().cloned().collect()
}

// Example usage
fn main() {
    let mut mappings = HashMap::new();

    let linux_config = HashMap::from([
        ("Kernel".to_string(), "5.10".to_string()),
        ("Shell".to_string(), "bash".to_string()),
    ]);

    let windows_config = HashMap::from([
        ("Version".to_string(), "10".to_string()),
        ("Shell".to_string(), "PowerShell".to_string()),
    ]);

    add_os_mapping(&mut mappings, "Linux", linux_config);
    add_os_mapping(&mut mappings, "Windows", windows_config);

    if let Some(configurations) = get_os_configurations(&mappings, "Linux") {
        println!("Linux configurations: {:?}", configurations);
    }

    let os_list = list_os_mappings(&mappings);
    println!("Supported OS: {:?}", os_list);
}