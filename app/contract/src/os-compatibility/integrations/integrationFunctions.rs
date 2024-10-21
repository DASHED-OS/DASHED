use std::collections::HashMap;

// Define a struct to represent an OS integration
#[derive(Debug, Clone)]
pub struct OSIntegration {
    pub os_name: String,
    pub features: Vec<String>,
}

// Function to add a new OS integration
pub fn add_os_integration(
    integrations: &mut HashMap<String, OSIntegration>, 
    os_name: &str, 
    features: Vec<String>
) {
    let integration = OSIntegration {
        os_name: os_name.to_string(),
        features,
    };
    integrations.insert(os_name.to_string(), integration);
}

// Function to get features of a specific OS integration
pub fn get_os_features(
    integrations: &HashMap<String, OSIntegration>, 
    os_name: &str
) -> Option<Vec<String>> {
    integrations.get(os_name).map(|integration| integration.features.clone())
}

// Function to list all OS integrations
pub fn list_os_integrations(integrations: &HashMap<String, OSIntegration>) -> Vec<String> {
    integrations.keys().cloned().collect()
}

// Example usage
fn main() {
    let mut integrations = HashMap::new();

    add_os_integration(&mut integrations, "Linux", vec!["Feature1".to_string(), "Feature2".to_string()]);
    add_os_integration(&mut integrations, "Windows", vec!["Feature3".to_string(), "Feature4".to_string()]);

    if let Some(features) = get_os_features(&integrations, "Linux") {
        println!("Linux features: {:?}", features);
    }

    let os_list = list_os_integrations(&integrations);
    println!("Supported OS: {:?}", os_list);
}