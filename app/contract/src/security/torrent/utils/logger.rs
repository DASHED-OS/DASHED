use log::{info, warn, error};

// Function to log information about a torrent integration
pub fn log_integration_info(feature_name: &str, message: &str) {
    info!("Integration: {}, Info: {}", feature_name, message);
}

// Function to log warnings related to a torrent integration
pub fn log_integration_warning(feature_name: &str, message: &str) {
    warn!("Integration: {}, Warning: {}", feature_name, message);
}

// Function to log errors related to a torrent integration
pub fn log_integration_error(feature_name: &str, message: &str) {
    error!("Integration: {}, Error: {}", feature_name, message);
}

// Function to log information about a report generation
pub fn log_report_info(report_type: &str, message: &str) {
    info!("Report: {}, Info: {}", report_type, message);
}

// Function to log warnings related to report generation
pub fn log_report_warning(report_type: &str, message: &str) {
    warn!("Report: {}, Warning: {}", report_type, message);
}

// Function to log errors related to report generation
pub fn log_report_error(report_type: &str, message: &str) {
    error!("Report: {}, Error: {}", report_type, message);
}

// Example usage
fn main() {
    env_logger::init();

    log_integration_info("proxy_redirection", "Integration completed successfully.");
    log_integration_warning("proxy_redirection", "Potential issue detected.");
    log_integration_error("proxy_redirection", "Integration failed due to configuration error.");

    log_report_info("detailed", "Report generated successfully.");
    log_report_warning("detailed", "Report generation took longer than expected.");
    log_report_error("detailed", "Failed to generate report due to missing data.");
}