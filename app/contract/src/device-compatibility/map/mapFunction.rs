use std::collections::HashMap;

// Define a struct to represent a device
#[derive(Debug, Clone)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub compatibility: Vec<String>,
}

// Function to map devices to their compatibility features
pub fn map_device_compatibility(devices: Vec<Device>) -> HashMap<String, Vec<String>> {
    let mut compatibility_map: HashMap<String, Vec<String>> = HashMap::new();

    for device in devices {
        compatibility_map.insert(device.id.clone(), device.compatibility.clone());
    }

    compatibility_map
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

    for (id, features) in compatibility_map {
        println!("Device ID: {}, Compatible Features: {:?}", id, features);
    }
}