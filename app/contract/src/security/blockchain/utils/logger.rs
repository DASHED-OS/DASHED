use log::{info, warn, error};

// Function to log information about a security check
pub fn log_security_check_info(check_name: &str, message: &str) {
    info!("Security Check: {}, Info: {}", check_name, message);
}

// Function to log warnings related to a security check
pub fn log_security_check_warning(check_name: &str, message: &str) {
    warn!("Security Check: {}, Warning: {}", check_name, message);
}

// Function to log errors related to a security check
pub fn log_security_check_error(check_name: &str, message: &str) {
    error!("Security Check: {}, Error: {}", check_name, message);
}

// Example usage
fn main() {
    env_logger::init();

    log_security_check_info