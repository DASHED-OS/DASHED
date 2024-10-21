use std::collections::HashMap;

// Define a struct to represent a security check result
#[derive(Debug, Clone)]
pub struct SecurityCheckResult {
    pub check_name: String,
    pub passed: bool,
    pub details: String,
}

// Function to perform a basic security check on a blockchain node
pub fn perform_security_check(node: &str) -> SecurityCheckResult {
    // Simulate a security check operation
    let passed = !node.contains("compromised");
    let details = if passed {
        format!("Node {} passed the security check.", node)
    } else {
        format!("Node {} failed the security check.", node)
    };

    SecurityCheckResult {
        check_name: "Basic Security Check".to_string(),
        passed,
        details,
    }
}

// Function to summarize security check results
pub fn summarize_security_results(results: &[SecurityCheckResult]) -> HashMap<String, usize> {
    let mut summary = HashMap::new();
    for result in results {
        let entry = summary.entry(result.check_name.clone()).or_insert(0);
        if result.passed {
            *entry += 1;
        }
    }
    summary
}

// Example usage
fn main() {
    let nodes = vec!["node1", "compromised_node", "node3"];
    let results: Vec<SecurityCheckResult> = nodes.iter().map(|&node| perform_security_check(node)).collect();

    for result in &results {
        println!("{:?}", result);
    }

    let summary = summarize_security_results(&results);
    println!("Security Check Summary: {:?}", summary);
}