use std::collections::HashMap;

// Define a struct to represent a torrent integration result
#[derive(Debug, Clone)]
pub struct TorrentIntegrationResult {
    pub feature_name: String,
    pub integrated: bool,
    pub details: String,
}

// Function to integrate a security feature into the Torrent browser
pub fn integrate_torrent_feature(feature: &str) -> TorrentIntegrationResult {
    // Simulate the integration process
    let integrated = !feature.contains("unsupported");
    let details = if integrated {
        format!("Security feature integrated into Torrent feature {}.", feature)
    } else {
        format!("Failed to integrate security feature into Torrent feature {}.", feature)
    };

    TorrentIntegrationResult {
        feature_name: feature.to_string(),
        integrated,
        details,
    }
}

// Function to summarize torrent integration results
pub fn summarize_torrent_integration_results(results: &[TorrentIntegrationResult]) -> HashMap<String, usize> {
    let mut summary = HashMap::new();
    for result in results {
        let entry = summary.entry(result.feature_name.clone()).or_insert(0);
        if result.integrated {
            *entry += 1;
        }
    }
    summary
}

// Example usage
fn main() {
    let features = vec!["proxy_redirection", "unsupported_feature", "encryption"];
    let results: Vec<TorrentIntegrationResult> = features.iter().map(|&feature| integrate_torrent_feature(feature)).collect();

    for result in &results {
        println!("{:?}", result);
    }

    let summary = summarize_torrent_integration_results(&results);
    println!("Torrent Integration Summary: {:?}", summary);
}