use log::{info, warn, error};

// Function to log information about a device
pub fn log_device_info(device_id: &str, message: &str) {
    info!("Device ID: {}, Info: {}", device_id, message);
}

// Function to log warnings related to a device
pub fn log_device_warning(device_id: &str, message: &str) {
    warn!("Device ID: {}, Warning: {}", device_id, message);
}

// Function to log errors related to a device
pub fn log_device_error(device_id: &str, message: &str) {
    error!("Device ID: {}, Error: {}", device_id, message);
}

// Example usage
fn main() {
    env_logger::init();

    log_device_info("device1", "Device initialized successfully.");
    log_device_warning("device1", "Device running low on memory.");
    log_device_error("device1", "Device failed to connect to network.");
}