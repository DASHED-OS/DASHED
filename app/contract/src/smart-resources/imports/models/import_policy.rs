use std::collections::HashMap;

// Define a struct to represent an import policy
#[derive(Debug, Clone)]
pub struct ImportPolicy {
    pub policy_name: String,
    pub formats_allowed: Vec<String>,
    pub max_file_size_mb: u32,
    pub additional_settings: HashMap<String, String>,
}

// Function to create a new import policy
pub fn create_import_policy(policy_name: &str, formats_allowed: Vec<String>, max_file_size_mb: u32) -> ImportPolicy {
    ImportPolicy {
        policy_name: policy_name.to_string(),
        formats_allowed,
        max_file_size_mb,
        additional_settings: HashMap::new(),
    }
}

// Function to add a setting to an import policy
pub fn add_policy_setting(policy: &mut ImportPolicy, key: &str, value: &str) {
    policy.additional_settings.insert(key.to_string(), value.to_string());
}

// Example usage
fn main() {
    let mut policy = create_import_policy("Default Import Policy", vec!["CSV".to_string(), "JSON".to_string()], 100);
    add_policy_setting(&mut policy, "compression", "gzip");
    println!("Import Policy: {:?}", policy);
}