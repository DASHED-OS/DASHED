use std::collections::HashMap;
use crate::security::torrent::modules::torrent_integrations::TorrentIntegrationResult;

// Function to generate a detailed report from torrent integration results
pub fn generate_detailed_report(results: &[TorrentIntegrationResult]) -> String {
    let mut report = String::from("Torrent Integration Detailed Report\n\n");

    for result in results {
        report.push_str(&format!(
            "Feature Name: {}\nIntegrated: {}\nDetails: {}\n\n",
            result.feature_name, result.integrated, result.details
        ));
    }

    report
}

// Function to generate a summary report from torrent integration results
pub fn generate_summary_report(results: &[TorrentIntegrationResult]) -> String {
    let summary = summarize_torrent_integration_results(results);
    let mut report = String::from("Torrent Integration Summary Report\n\n");

    for (feature_name, integrated_count) in summary {
        report.push_str(&format!(
            "Feature Name: {}\nIntegrated Features: {}\n\n",
            feature_name, integrated_count
        ));
    }

    report
}

// Example usage
fn main() {
    let results = vec![
        TorrentIntegrationResult {
            feature_name