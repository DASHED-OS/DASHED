use log::{info, warn, error};

// Function to log information about a security enhancement
pub fn log_enhancement_info(enhancement_name: &str, message: &str) {
    info!("Enhancement: {}, Info: {}", enhancement_name, message);
}

// Function to log warnings related to a security enhancement
pub fn log_enhancement_warning(enhancement_name: &str, message: &str) {
    warn!("Enhancement: {}, Warning: {}", enhancement_name, message);
}

// Function to log errors related to a security enhancement
pub fn log_enhancement_error(enhancement_name: &str, message: &str) {
    error!("Enhancement: {}, Error: {}", enhancement_name, message);
}

// Function to log information about an integration
pub fn log_integration_info(component_name: &str, message: &str) {
    info!("Integration: {}, Info: {}", component_name, message);
}

// Function to log warnings related to an integration
pub fn log_integration_warning(component_name: &str, message: &str) {
    warn!("Integration: {}, Warning: {}", component_name, message);
}

// Function to log errors related to an integration
pub fn log_integration_error(component_name: &str, message: &str) {
    error!("Integration: {}, Error: {}", component_name, message);
}

// Example usage
fn main() {
    env_logger::init();

    log_enhancement_info("Basic Security Enhancement", "Enhancement applied successfully.");
    log_enhancement_warning("Basic Security Enhancement", "Potential issue detected.");
    log_enhancement_error("Basic Security Enhancement", "Enhancement failed due to configuration error.");

    log_integration_info("network_manager", "Integration completed successfully.");
    log_integration_warning("network_manager", "Potential compatibility issue detected.");
    log_integration_error("network_manager", "Integration failed due to missing dependencies.");
}