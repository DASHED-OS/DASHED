use crate::security::tailsOS::modules::security_enhancements::SecurityEnhancementResult;
use crate::security::tailsOS::modules::tails_integration::IntegrationResult;

// Function to filter successful security enhancements
pub fn filter_successful_enhancements(results: &[SecurityEnhancementResult]) -> Vec<SecurityEnhancementResult> {
    results.iter().cloned().filter(|result| result.applied).collect()
}

// Function to count the number of successful security enhancements
pub fn count_successful_enhancements(results: &[SecurityEnhancementResult]) -> usize {
    results.iter().filter(|result| result.applied).count()
}

// Function to find a security enhancement result by enhancement name
pub fn find_enhancement_by_name(results: &[SecurityEnhancementResult], enhancement_name: &str) -> Option<SecurityEnhancementResult> {
    results.iter().cloned().find(|result| result.enhancement_name == enhancement_name)
}

// Function to filter successful integrations
pub fn filter_successful_integrations(results: &[IntegrationResult]) -> Vec<IntegrationResult> {
    results.iter().cloned().filter(|result| result.integrated).collect()
}

// Function to count the number of successful integrations
pub fn count_successful_integrations(results: &[IntegrationResult]) -> usize {
    results.iter().filter(|result| result.integrated).count()
}

// Function to find an integration result by component name
pub fn find_integration_by_name(results: &[IntegrationResult], component_name: &str) -> Option<IntegrationResult> {
    results.iter().cloned().find(|result| result.component_name == component_name)
}

// Example usage
fn main() {
    let enhancement_results = vec![
        SecurityEnhancementResult {
            enhancement_name: "Basic Security Enhancement".to_string(),
            applied: true,
            details: "Security enhancement applied to component1.".to_string(),
        },
        SecurityEnhancementResult {
            enhancement_name: "Basic Security Enhancement".to_string(),
            applied: false,
            details: "Failed to apply security enhancement to outdated_component.".to_string(),
        },
    ];

    let integration_results = vec![
        IntegrationResult {
            component_name: "network_manager".to_string(),
            integrated: true,
            details: "Security feature integrated into network_manager.".to_string(),
        },
        IntegrationResult {
            component_name: "unsupported_component".to_string(),
            integrated: false,
            details: "Failed to integrate security feature into unsupported_component.".to_string(),
        },
    ];

    let successful_enhancements = filter_successful_enhancements(&enhancement_results);
    println!("Successful Enhancements: {:?}", successful_enhancements);

    let enhancement_count = count_successful_enhancements(&enhancement_results);
    println!("Number of Successful Enhancements: {}", enhancement_count);

    if let Some(result) = find_enhancement_by_name(&enhancement_results, "Basic Security Enhancement") {
        println!("Found Enhancement: {:?}", result);
    }

    let successful_integrations = filter_successful_integrations(&integration_results);
    println!("Successful Integrations: {:?}", successful_integrations);

    let integration_count = count_successful_integrations(&integration_results);
    println!("Number of Successful Integrations: {}", integration_count);

    if let Some(result) = find_integration_by_name(&integration_results, "network_manager") {
        println!("Found Integration: {:?}", result);
    }
}