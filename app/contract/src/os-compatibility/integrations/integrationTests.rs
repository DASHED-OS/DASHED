#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_add_os_integration() {
        let mut integrations = HashMap::new();
        add_os_integration(&mut integrations, "Linux", vec!["Feature1".to_string(), "Feature2".to_string()]);

        assert!(integrations.contains_key("Linux"));
        assert_eq!(integrations["Linux"].features, vec!["Feature1".to_string(), "Feature2".to_string()]);
    }

    #[test]
    fn test_get_os_features() {
        let mut integrations = HashMap::new();
        add_os_integration(&mut integrations, "Windows", vec!["Feature3".to_string(), "Feature4".to_string()]);

        let features = get_os_features(&integrations, "Windows");
        assert_eq!(features, Some(vec!["Feature3".to_string(), "Feature4".to_string()]));
    }

    #[test]
    fn test_list_os_integrations() {
        let mut integrations = HashMap::new();
        add_os_integration(&mut integrations, "Linux", vec!["Feature1".to_string()]);
        add_os_integration(&mut integrations, "Windows", vec!["Feature2".to_string()]);

        let os_list = list_os_integrations(&integrations);
        assert_eq!(os_list, vec!["Linux".to_string(), "Windows".to_string()]);
    }
}