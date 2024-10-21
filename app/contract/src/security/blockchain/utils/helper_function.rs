use crate::security::ai_pentests::pentest::ai_pentest_functions::PentestResult;
use crate::security::blockchain::modules::security_checks::SecurityCheckResult;

// Function to filter successful security check results
pub fn filter_successful_security_checks(results: &[SecurityCheckResult]) -> Vec<SecurityCheckResult> {
    results.iter().cloned().filter(|result| result.passed).collect()
}

// Function to count the number of successful security checks
pub fn count_successful_security_checks(results: &[SecurityCheckResult]) -> usize {
    results.iter().filter(|result| result.passed).count()
}

// Function to find a security check result by check name
pub fn find_security_check_by_name(results: &[SecurityCheckResult], check_name: &str) -> Option<SecurityCheckResult> {
    results.iter().cloned().find(|result| result.check_name == check_name)
}

// Example usage
fn main() {
    let security_results = vec![
        SecurityCheckResult {
            check_name: "Basic Security Check".to_string(),
            passed: true,
            details: "Node node1 passed the security check.".to_string(),
        },
        SecurityCheckResult {
            check_name: "Basic Security Check".to_string(),
            passed: false,
            details: "Node compromised_node failed the security check.".to_string(),
        },
    ];

    let successful_checks = filter_successful_security_checks(&security_results);
    println!("Successful Security Checks: {:?}", successful_checks);

    let success_count = count_successful_security_checks(&security_results);
    println!("Number of Successful Security Checks: {}", success_count);

    if let Some(result) = find_security_check_by_name(&security_results, "Basic Security Check") {
        println!("Found Security Check: {:?}", result);
    }
}