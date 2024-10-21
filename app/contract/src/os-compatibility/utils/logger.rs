use log::{info, warn, error};

// Function to log information about OS compatibility
pub fn log_os_info(os_name: &str, message: &str) {
    info!("OS: {}, Info: {}", os_name, message);
}

// Function to log warnings related to OS compatibility
pub fn log_os_warning(os_name: &str, message: &str) {
    warn!("OS: {}, Warning: {}", os_name, message);
}

// Function to log errors related to OS compatibility
pub fn log_os_error(os_name: &str, message: &str) {
    error!("OS: {}, Error: {}", os_name, message);
}

// Example usage
fn main() {
    env_logger::init();

    log_os_info("Linux", "OS initialized successfully.");
    log_os_warning("Linux", "Potential compatibility issue detected.");
    log_os_error("Linux", "Failed to load OS-specific configuration.");
}