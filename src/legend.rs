use crate::error::LegendError;
use std::collections::HashMap;

pub struct AnonymizationMap {
    pub users: HashMap<String, String>,
    pub channels: HashMap<String, String>,
    pub emails: HashMap<String, String>,
    pub urls: HashMap<String, String>,
    pub keywords: HashMap<String, String>,
    pub display_names: HashMap<String, String>,
}

impl AnonymizationMap {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            channels: HashMap::new(),
            emails: HashMap::new(),
            urls: HashMap::new(),
            keywords: HashMap::new(),
            display_names: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.users.is_empty()
            && self.channels.is_empty()
            && self.emails.is_empty()
            && self.urls.is_empty()
            && self.keywords.is_empty()
            && self.display_names.is_empty()
    }
}

pub fn format_legend(map: &AnonymizationMap) -> Result<String, LegendError> {
    if map.is_empty() {
        return Ok(String::new());
    }

    let mut legend = String::new();
    legend.push_str("\n=== ANONYMIZATION LEGEND ===\n");

    // Sort entries for consistent output
    let mut all_entries: Vec<(String, String)> = Vec::new();

    // Collect all mappings
    for (original, anonymous) in &map.users {
        all_entries.push((original.clone(), anonymous.clone()));
    }

    for (original, anonymous) in &map.channels {
        all_entries.push((original.clone(), anonymous.clone()));
    }

    for (original, anonymous) in &map.emails {
        all_entries.push((original.clone(), anonymous.clone()));
    }

    for (original, anonymous) in &map.urls {
        all_entries.push((original.clone(), anonymous.clone()));
    }

    for (original, anonymous) in &map.keywords {
        all_entries.push((original.clone(), anonymous.clone()));
    }

    for (original, anonymous) in &map.display_names {
        all_entries.push((original.clone(), anonymous.clone()));
    }

    // Sort by anonymous name for consistent output
    all_entries.sort_by(|a, b| a.1.cmp(&b.1));

    // Format entries
    for (original, anonymous) in all_entries {
        legend.push_str(&format!("{} → {}\n", original, anonymous));
    }

    Ok(legend)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_legend() {
        let map = AnonymizationMap::new();
        let legend = format_legend(&map).unwrap();
        assert!(legend.is_empty());
    }

    #[test]
    fn test_format_legend() {
        let mut map = AnonymizationMap::new();
        map.users
            .insert("@john.doe".to_string(), "@user1".to_string());
        map.channels
            .insert("#general".to_string(), "#ch1".to_string());
        map.emails.insert(
            "test@example.com".to_string(),
            "user1@domain1.com".to_string(),
        );

        let legend = format_legend(&map).unwrap();

        assert!(legend.contains("=== ANONYMIZATION LEGEND ==="));
        assert!(legend.contains("@john.doe → @user1"));
        assert!(legend.contains("#general → #ch1"));
        assert!(legend.contains("test@example.com → user1@domain1.com"));
    }

    #[test]
    fn test_legend_sorting() {
        let mut map = AnonymizationMap::new();
        map.users.insert("@beta".to_string(), "@user2".to_string());
        map.users.insert("@alpha".to_string(), "@user1".to_string());

        let legend = format_legend(&map).unwrap();
        let lines: Vec<&str> = legend.lines().collect();

        // Find the position of each user in the legend
        let user1_pos = lines.iter().position(|line| line.contains("@user1"));
        let user2_pos = lines.iter().position(|line| line.contains("@user2"));

        assert!(user1_pos < user2_pos);
    }
}

