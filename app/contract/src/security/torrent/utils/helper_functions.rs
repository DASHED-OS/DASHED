use crate::security::torrent::modules::torrent_integrations::TorrentIntegrationResult;

// Function to filter successful torrent integrations
pub fn filter_successful_integrations(results: &[TorrentIntegrationResult]) -> Vec<TorrentIntegrationResult> {
    results.iter().cloned().filter(|result| result.integrated).collect()
}

// Function to count the number of successful torrent integrations
pub fn count_successful_integrations(results: &[TorrentIntegrationResult]) -> usize {
    results.iter().filter(|result| result.integrated).count()
}

// Function to find a torrent integration result by feature name
pub fn find_integration_by_name(results: &[TorrentIntegrationResult], feature_name: &str) -> Option<TorrentIntegrationResult> {
    results.iter().cloned().find(|result| result.feature_name == feature_name)
}

// Example usage
fn main() {
    let results = vec![
        TorrentIntegrationResult {
            feature_name: "proxy_redirection".to_string(),
            integrated: true,
            details: "Security feature integrated into proxy_redirection.".to_string(),
        },
        TorrentIntegrationResult {
            feature_name: "unsupported_feature".to_string(),
            integrated: false,
            details: "Failed to integrate security feature into unsupported_feature.".to_string(),
        },
    ];

    let successful_integrations = filter_successful_integrations(&results);
    println!("Successful Integrations: {:?}", successful_integrations);

    let integration_count = count_successful_integrations(&results);
    println!("Number of Successful Integrations: {}", integration_count);

    if let Some(result) = find_integration_by_name(&results, "proxy_redirection") {
        println!("Found Integration: {:?}", result);
    }
}