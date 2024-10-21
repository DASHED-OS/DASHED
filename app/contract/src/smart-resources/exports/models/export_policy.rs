use std::collections::HashMap;

// Define a struct to represent an export policy
#[derive(Debug, Clone)]
pub struct ExportPolicy {
    pub policy_name: String,
    pub formats_allowed: Vec<String>,
    pub max_file_size_mb: u32,
    pub additional_settings: HashMap<String, String>,
}

// Function to create a new export policy
pub fn create_export_policy(policy_name: &str, formats_allowed: Vec<String>, max_file_size_mb: u32) -> ExportPolicy {
    ExportPolicy {
        policy_name: policy_name.to_string(),
        formats_allowed,
        max_file_size_mb,
        additional_settings: HashMap::new(),
    }
}

// Function to add a setting to an export policy
pub fn add_policy_setting(policy: &mut ExportPolicy, key: &str, value: &str) {
    policy.additional_settings.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut policy = create_export_policy("Default Policy", vec!["CSV".to_string(), "JSON".to_string()], 100);
    add_policy_setting(&mut policy, "compression", "gzip");
    println!("Export Policy: {:?}", policy);
}