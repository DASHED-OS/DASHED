pub struct UserSettings {
    pub enable_security_checks: bool,
    pub notification_level: NotificationLevel,
    pub custom_rules: Vec<String>,
}

pub enum NotificationLevel {
    Low,
    Medium,
    High,
}

impl UserSettings {
    pub fn new() -> Self {
        UserSettings {
            enable_security_checks: true,
            notification_level: NotificationLevel::Medium,
            custom_rules: Vec::new(),
        }
    }

    pub fn set_security_checks(&mut self, enable: bool) {
        self.enable_security_checks = enable;
    }

    pub fn set_notification_level(&mut self, level: NotificationLevel) {
        self.notification_level = level;
    }

    pub fn add_custom_rule(&mut self, rule: String) {
        self.custom_rules.push(rule);
    }

    pub fn remove_custom_rule(&mut self, rule: &str) {
        self.custom_rules.retain(|r| r != rule);
    }
}