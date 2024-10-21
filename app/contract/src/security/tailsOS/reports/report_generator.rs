use std::collections::HashMap;
use crate::security::tailsOS::modules::security_enhancements::SecurityEnhancementResult;

// Function to generate a detailed report from security enhancement results
pub fn generate_detailed_report(results: &[SecurityEnhancementResult]) -> String {
    let mut report = String::from("TailsOS Security Enhancement Detailed Report\n\n");

    for result in results {
        report.push_str(&format!(
            "Enhancement Name: {}\nApplied: {}\nDetails: {}\n\n",
            result.enhancement_name, result.applied, result.details
        ));
    }

    report
}

// Function to generate a summary report from security enhancement results
pub fn generate_summary_report(results: &[SecurityEnhancementResult]) -> String {
    let summary = summarize_enhancement_results(results);
    let mut report = String::from("TailsOS Security Enhancement Summary Report\n\n");

    for (enhancement_name, applied_count) in summary {
        report.push_str(&format!(
            "Enhancement Name: {}\nApplied Enhancements: {}\n\n",
            enhancement_name, applied_count
        ));
    }

    report
}

// Example usage
fn main() {
    let results = vec![
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

    let detailed_report = generate_detailed_report(&results);
    println!("{}", detailed_report);

    let summary_report = generate_summary_report(&results);
    println!("{}", summary_report);
}