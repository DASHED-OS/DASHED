use std::collections::HashMap;
use crate::device_compatibility::map::mapFunction::Device;

// Function to check if a device supports a specific feature
pub fn is_device_compatible(device: &Device, feature: &str) -> bool {
    device.compatibility.contains(&feature.to_string())
}

// Function to get a list of all features supported by a collection of devices
pub fn get_all_supported_features(devices: &[Device]) -> Vec<String> {
    let mut features = Vec::new();
    for device in devices {
        for feature in &device.compatibility {
            if !features.contains(feature) {
                features.push(feature.clone());
            }
        }
    }
    features
}

// Function to create a compatibility map from a list of devices
pub fn create_compatibility_map(devices: Vec<Device>) -> HashMap<String, Vec<String>> {
    devices.into_iter()
        .map(|device| (device.id.clone(), device.compatibility.clone()))
        .collect()
}