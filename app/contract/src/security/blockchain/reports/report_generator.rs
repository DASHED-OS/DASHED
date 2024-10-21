use std::collections::HashMap;
use crate::security::ai_pentests::pentest::ai_pentest_functions::PentestResult;
use crate::security::blockchain::modules::blockchain_security::SecurityCheckResult;

// Function to generate a detailed report from pentest and security check results
pub fn generate_detailed_report(pentest_results: &[PentestResult], security_results: &[SecurityCheckResult]) -> String {
    let mut report = String::from("Detailed Security Report\n\n");

    report.push_str("AI Pentest Detailed Report\n");
    for result in pentest_results {
        report.push_str(&format!(
            "Test Name: {}\nSuccess: {}\nDetails: {}\n\n",
            result.test_name, result.success, result.details
        ));
    }

    report.push_str("Blockchain Security Detailed Report\n");
    for result in security_results {
        report.push_str(&format!(
            "Check Name: {}\nPassed: {}\nDetails: {}\n\n",
            result.check_name, result.passed, result.details
        ));
    }

    report
}

// Function to generate a summary report from pentest and security check results
pub fn generate_summary_report(pentest_results: &[PentestResult], security_results: &[SecurityCheckResult]) -> String {
    let pentest_summary = summarize_pentest_results(pentest_results);
    let security_summary = summarize_security_results(security_results);

    let mut report = String::from("Summary Security Report\n\n");

    report.push_str("AI Pentest Summary Report\n");
    for (test_name, success_count) in pentest_summary {
        report.push_str(&format!(
            "Test Name: {}\nSuccessful Tests: {}\n\n",
            test_name, success_count
        ));
    }

    report.push_str("Blockchain Security Summary Report\n");
    for (check_name, pass_count) in security_summary {
        report.push_str(&format!(
            "Check Name: {}\nPassed Checks: {}\n\n",
            check_name, pass_count
        ));
    }

    report
}

// Example usage
fn main() {
    let pentest_results = vec![
        PentestResult {
            test_name: "Basic AI Pentest".to_string(),