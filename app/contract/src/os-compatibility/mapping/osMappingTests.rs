#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_add_os_mapping() {
        let mut mappings = HashMap::new();
        let linux_config = HashMap::from([
            ("Kernel".to_string(), "5.10".to_string()),
            ("Shell".to_string(), "bash".to_string()),
        ]);

        add_os_mapping(&mut mappings, "Linux", linux_config.clone());

        assert!(mappings.contains_key("Linux"));
        assert_eq!(mappings["Linux"].configurations, linux_config);
    }

    #[test]
    fn test_get_os_configurations() {
        let mut mappings = HashMap::new();
        let windows_config = HashMap::from([
            ("Version".to_string(), "10".to_string()),
            ("Shell".to_string(), "PowerShell".to_string()),
        ]);

        add_os_mapping(&mut mappings, "Windows", windows_config.clone());

        let configurations = get_os_configurations(&mappings, "Windows");
        assert_eq!(configurations, Some(windows_config));
    }

    #[test]
    fn test_list_os_mappings() {
        let mut mappings = HashMap::new();
        add_os_mapping(&mut mappings, "Linux", HashMap::new());
        add_os_mapping(&mut mappings, "Windows", HashMap::new());

        let os_list = list_os_mappings(&mappings);
        assert_eq!(os_list, vec!["Linux".to_string(), "Windows".to_string()]);
    }
}