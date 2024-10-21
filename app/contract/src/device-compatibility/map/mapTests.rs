#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_map_device_compatibility() {
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

        let expected_map: HashMap<String, Vec<String>> = [
            ("device1".to_string(), vec!["featureA".to_string(), "featureB".to_string()]),
            ("device2".to_string(), vec!["featureB".to_string(), "featureC".to_string()]),
        ].iter().cloned().collect();

        let result_map = map_device_compatibility(devices);

        assert_eq!(result_map, expected_map);
    }
}