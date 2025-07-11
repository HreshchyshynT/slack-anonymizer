use crate::error::AnonymizationError;
use crate::legend::AnonymizationMap;
use crate::patterns::{
    anonymize_channels, anonymize_display_names, anonymize_emails, anonymize_keywords,
    anonymize_urls, anonymize_users,
};

pub struct Options {
    pub anonymize_urls: bool,
    pub keywords: Vec<String>,
}

impl Options {
    pub fn default() -> Self {
        Self {
            anonymize_urls: false,
            keywords: Vec::new(),
        }
    }
    pub fn new(anonymize_urls: bool, keywords: Vec<String>) -> Self {
        Self {
            anonymize_urls,
            keywords,
        }
    }
}

pub fn anonymize_text(
    text: &str,
    options: &Options,
) -> Result<(String, AnonymizationMap), AnonymizationError> {
    let mut map = AnonymizationMap::new();
    let mut result = text.to_string();

    // Process in the specified order:
    // 1. User mentions - to avoid conflicts with display names,
    // because usernames could be copied in format @Name Format
    // when user just select text in slack and copied it
    result = anonymize_users(&result, &mut map.users)?;

    // 2. Display names
    result = anonymize_display_names(&result, &mut map.displayNames)?;

    // 3. Channel references
    result = anonymize_channels(&result, &mut map.channels)?;

    // 4. Email addresses
    result = anonymize_emails(&result, &mut map.emails)?;

    // 5. URLs (if enabled)
    if options.anonymize_urls {
        result = anonymize_urls(&result, &mut map.urls)?;
    }

    // 6. Custom keywords
    if !options.keywords.is_empty() {
        result = anonymize_keywords(&result, &options.keywords, &mut map.keywords)?;
    }

    Ok((result, map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_anonymization() {
        let text = "Hey @john, check #general and email test@example.com";
        let options = Options::default();

        let (result, map) = anonymize_text(text, &options).unwrap();
        println!("Result: {}", result);

        assert!(result.contains("@user1"));
        assert!(result.contains("#ch1"));
        assert!(result.contains("user1@domain1.com"));
        assert!(!map.is_empty());
    }

    #[test]
    fn test_with_urls() {
        let text = "Visit https://company.com for more info";
        let options = Options::new(true, vec![]);

        let (result, map) = anonymize_text(text, &options).unwrap();

        assert!(result.contains("https://example1.com"));
        assert!(!map.urls.is_empty());
    }

    #[test]
    fn test_with_keywords() {
        let text = "ProjectX is ready and ClientABC approved";
        let keywords = vec!["ProjectX".to_string(), "ClientABC".to_string()];
        let options = Options::new(false, keywords);

        let (result, map) = anonymize_text(text, &options).unwrap();

        assert!(result.contains("keyword1"));
        assert!(result.contains("keyword2"));
        assert!(!map.keywords.is_empty());
    }

    #[test]
    fn test_processing_order() {
        // Test that user mentions are processed before keywords
        let text = "Contact @support about ProjectX";
        let keywords = vec!["support".to_string()];
        let options = Options::new(false, keywords);

        let (result, _) = anonymize_text(text, &options).unwrap();

        // @support should be anonymized as @user1, not affected by keyword replacement
        assert!(result.contains("@user1"));
        assert!(!result.contains("keyword1"));
        assert!(!result.contains("@keyword1"));
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let options = Options::default();

        let (result, map) = anonymize_text(text, &options).unwrap();

        assert_eq!(result, "");
        assert!(map.is_empty());
    }

    #[test]
    fn test_complex_text() {
        let text = r#"
        Hey @john.doe and @jane_smith!
        
        Please check #general and #dev-team channels.
        Contact support@company.com or sales@client.org
        Visit https://company.com/docs and http://client.org/help
        
        ProjectX needs review and ClientABC approved it.
        The SecretFeature is ready for testing.
        "#;

        let keywords = vec![
            "ProjectX".to_string(),
            "ClientABC".to_string(),
            "SecretFeature".to_string(),
        ];
        let options = Options::new(true, keywords);

        let (result, map) = anonymize_text(text, &options).unwrap();

        // Check that all types were anonymized
        assert!(!map.users.is_empty());
        assert!(!map.channels.is_empty());
        assert!(!map.emails.is_empty());
        assert!(!map.urls.is_empty());
        assert!(!map.keywords.is_empty());

        // Check specific patterns
        assert!(result.contains("@user"));
        assert!(result.contains("#ch"));
        assert!(result.contains("@domain"));
        assert!(result.contains("https://example"));
        assert!(result.contains("keyword"));
    }
}

