use std::collections::HashMap;

// Define a struct to represent a security enhancement result
#[derive(Debug, Clone)]
pub struct SecurityEnhancementResult {
    pub enhancement_name: String,
    pub applied: bool,
    pub details: String,
}

// Function to apply a security enhancement to a TailsOS component
pub fn apply_security_enhancement(component: &str) -> SecurityEnhancementResult {
    // Simulate applying a security enhancement
    let applied = !component.contains("outdated");
    let details = if applied {
        format!("Security enhancement applied to component {}.", component)
    } else {
        format!("Failed to apply security enhancement to component {}.", component)
    };

    SecurityEnhancementResult {
        enhancement_name: "Basic Security Enhancement".to_string(),
        applied,
        details,
    }
}

// Function to summarize security enhancement results
pub fn summarize_enhancement_results(results: &[SecurityEnhancementResult]) -> HashMap<String, usize> {
    let mut summary = HashMap::new();
    for result in results {
        let entry = summary.entry(result.enhancement_name.clone()).or_insert(0);
        if result.applied {
            *entry += 1;
        }
    }
    summary
}

// Example usage
fn main() {
    let components = vec!["component1", "outdated_component", "component3"];
    let results: Vec<SecurityEnhancementResult> = components.iter().map(|&component| apply_security_enhancement(component)).collect();

    for result in &results {
        println!("{:?}", result);
    }

    let summary = summarize_enhancement_results(&results);
    println!("Security Enhancement Summary: {:?}", summary);
}