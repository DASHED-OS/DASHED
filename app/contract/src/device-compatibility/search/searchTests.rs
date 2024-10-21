#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::device_compatibility::map::mapFunction::Device;

    #[test]
    fn test_search_devices_by_feature() {
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

        let result = search_devices_by_feature(&compatibility_map, "featureB");
        assert_eq!(result, vec!["device1".to_string(), "device2".to_string()]);
    }

    #[test]
    fn test_search_features_by_device() {
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

        let result = search_features_by_device(&compatibility_map, "device1");
        assert_eq!(result, Some(vec!["featureA".to_string(), "featureB".to_string()]));
    }
}