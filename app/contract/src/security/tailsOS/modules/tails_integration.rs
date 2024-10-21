use std::collections::HashMap;

// Define a struct to represent an integration result
#[derive(Debug, Clone)]
pub struct IntegrationResult {
    pub component_name: String,
    pub integrated: bool,
    pub details: String,
}

// Function to integrate a security feature into a TailsOS component
pub fn integrate_security_feature(component: &str) -> IntegrationResult {
    // Simulate the integration process
    let integrated = !component.contains("unsupported");
    let details = if integrated {
        format!("Security feature integrated into component {}.", component)
    } else {
        format!("Failed to integrate security feature into component {}.", component)
    };

    IntegrationResult {
        component_name: component.to_string(),
        integrated,
        details,
    }
}

// Function to summarize integration results
pub fn summarize_integration_results(results: &[IntegrationResult]) -> HashMap<String, usize> {
    let mut summary = HashMap::new();
    for result in results {
        let entry = summary.entry(result.component_name.clone()).or_insert(0);
        if result.integrated {
            *entry += 1;
        }
    }
    summary
}

// Example usage
fn main() {
    let components = vec!["network_manager", "unsupported_component", "firewall"];
    let results: Vec<IntegrationResult> = components.iter().map(|&component| integrate_security_feature(component)).collect();

    for result in &results {
        println!("{:?}", result);
    }

    let summary = summarize_integration_results(&results);
    println!("Integration Summary: {:?}", summary);
}