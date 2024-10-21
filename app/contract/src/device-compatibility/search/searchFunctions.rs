use std::collections::HashMap;
use crate::device_compatibility::map::mapFunction::Device;

// Function to search for devices compatible with a specific feature
pub fn search_devices_by_feature(
    devices: &HashMap<String, Vec<String>>, 
    feature: &str
) -> Vec<String> {
    devices.iter()
        .filter_map(|(id, features)| {
            if features.contains(&feature.to_string()) {
                Some(id.clone())
            } else {
                None
            }
        })
        .collect()
}

// Function to search for features compatible with a specific device
pub fn search_features_by_device(
    devices: &HashMap<String, Vec<String>>, 
    device_id: &str
) -> Option<Vec<String>> {
    devices.get(device_id).cloned()
}

// Example usage
fn main() {
    let devices = vec![
        Device {
            id: "device1".to_string(),
            name: "Device One".to_string(),
            compatibility: vec!["featureA".to_string(), "featureB".to_string()],
        },
        Device {
            id: "device2".to_string(),
            name: "Device Two".to_string(),
            compatibility: vec!["featureB".to_string(), "featureC".to_string()],
        },
    ];

    let compatibility_map = map_device_compatibility(devices);

    let devices_with_feature_b = search_devices_by_feature(&compatibility_map, "featureB");
    println!("Devices with featureB: {:?}", devices_with_feature_b);

    if let Some(features) = search_features_by_device(&compatibility_map, "device1") {
        println!("Features for device1: {:?}", features);
    } else {
        println!("Device not found");
    }
}