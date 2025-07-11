use crate::error::PatternError;
use regex::Regex;
use std::collections::HashMap;

// Regex patterns
const USER_PATTERN: &str =
    r"(^|[^a-zA-Z0-9._%+-])@([a-z0-9._-]{1,21}|[A-Z][a-zA-Z]+\s+[A-Z][a-zA-Z]+)([^a-z0-9._A-Z-]|$)";
const CHANNEL_PATTERN: &str = r"#[a-zA-Z0-9._-]+";
const EMAIL_PATTERN: &str = r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}";
const URL_PATTERN: &str = r"https?://[^\s]+";
// Pattern for display names - exactly two words starting with uppercase letters
const DISPLAY_NAME_PATTERN: &str = r"\b[A-Z][a-zA-Z]+\s+[A-Z][a-zA-Z]+\b";

pub fn anonymize_users(
    text: &str,
    map: &mut HashMap<String, String>,
) -> Result<String, PatternError> {
    let re = Regex::new(USER_PATTERN)?;
    let mut counter = map.len() + 1;

    let result = re.replace_all(text, |caps: &regex::Captures| {
        let before_char = caps.get(1).unwrap().as_str();
        let username_part = caps.get(2).unwrap().as_str();
        let following_char = caps.get(3).map_or("", |m| m.as_str());

        // Handle both @username and @Name Surname cases
        let clean_mention = if username_part.contains(' ') {
            // This is @Name Surname format
            format!("@{}", username_part)
        } else {
            // This is @username format - remove trailing periods
            let clean_username = username_part.trim_end_matches('.');
            format!("@{}", clean_username)
        };

        if let Some(anonymous) = map.get(&clean_mention) {
            // Handle trailing period preservation for @username format
            if !username_part.contains(' ')
                && username_part.ends_with('.')
                && !clean_mention.ends_with('.')
            {
                format!("{}{}.{}", before_char, anonymous, following_char)
            } else {
                format!("{}{}{}", before_char, anonymous, following_char)
            }
        } else {
            let anonymous = format!("@user{}", counter);
            map.insert(clean_mention.clone(), anonymous.clone());
            counter += 1;

            // Handle trailing period preservation for @username format
            if !username_part.contains(' ')
                && username_part.ends_with('.')
                && !clean_mention.ends_with('.')
            {
                format!("{}{}.{}", before_char, anonymous, following_char)
            } else {
                format!("{}{}{}", before_char, anonymous, following_char)
            }
        }
    });

    Ok(result.to_string())
}

pub fn anonymize_channels(
    text: &str,
    map: &mut HashMap<String, String>,
) -> Result<String, PatternError> {
    let re = Regex::new(CHANNEL_PATTERN)?;
    let mut counter = map.len() + 1;

    let result = re.replace_all(text, |caps: &regex::Captures| {
        let matched = caps.get(0).unwrap().as_str();

        if let Some(anonymous) = map.get(matched) {
            anonymous.clone()
        } else {
            let anonymous = format!("#ch{}", counter);
            map.insert(matched.to_string(), anonymous.clone());
            counter += 1;
            anonymous
        }
    });

    Ok(result.to_string())
}

pub fn anonymize_emails(
    text: &str,
    map: &mut HashMap<String, String>,
) -> Result<String, PatternError> {
    let re = Regex::new(EMAIL_PATTERN)?;
    let mut counter = map.len() + 1;

    let result = re.replace_all(text, |caps: &regex::Captures| {
        let matched = caps.get(0).unwrap().as_str();

        if let Some(anonymous) = map.get(matched) {
            anonymous.clone()
        } else {
            let anonymous = format!("user{}@domain{}.com", counter, counter);
            map.insert(matched.to_string(), anonymous.clone());
            counter += 1;
            anonymous
        }
    });

    Ok(result.to_string())
}

pub fn anonymize_urls(
    text: &str,
    map: &mut HashMap<String, String>,
) -> Result<String, PatternError> {
    let re = Regex::new(URL_PATTERN)?;
    let mut counter = map.len() + 1;

    let result = re.replace_all(text, |caps: &regex::Captures| {
        let matched = caps.get(0).unwrap().as_str();

        if let Some(anonymous) = map.get(matched) {
            anonymous.clone()
        } else {
            // Extract path from original URL if present
            let path = if let Some(protocol_pos) = matched.find("://") {
                let after_protocol = &matched[protocol_pos + 3..];
                if let Some(slash_pos) = after_protocol.find('/') {
                    &after_protocol[slash_pos..]
                } else {
                    ""
                }
            } else {
                ""
            };

            let anonymous = format!("https://example{}.com{}", counter, path);
            map.insert(matched.to_string(), anonymous.clone());
            counter += 1;
            anonymous
        }
    });

    Ok(result.to_string())
}

pub fn anonymize_display_names(
    text: &str,
    map: &mut HashMap<String, String>,
) -> Result<String, PatternError> {
    let re = Regex::new(DISPLAY_NAME_PATTERN)?;
    let mut counter = map.len() + 1;

    let result = re.replace_all(text, |caps: &regex::Captures| {
        let matched = caps.get(0).unwrap().as_str();

        if let Some(anonymous) = map.get(matched) {
            anonymous.clone()
        } else {
            let anonymous = format!("name{}", counter);
            map.insert(matched.to_string(), anonymous.clone());
            counter += 1;
            anonymous
        }
    });

    Ok(result.to_string())
}

pub fn anonymize_keywords(
    text: &str,
    keywords: &[String],
    map: &mut HashMap<String, String>,
) -> Result<String, PatternError> {
    let mut result = text.to_string();
    let mut counter = map.len() + 1;

    for keyword in keywords {
        if keyword.trim().is_empty() {
            continue;
        }

        // Create regex for whole word matching (case-insensitive)
        let pattern = format!(r"\b{}\b", regex::escape(keyword));
        let re = Regex::new(&format!("(?i){}", pattern))?;

        result = re
            .replace_all(&result, |caps: &regex::Captures| {
                let matched = caps.get(0).unwrap().as_str();
                let key = matched.to_lowercase(); // Use lowercase for consistency

                if let Some(anonymous) = map.get(&key) {
                    anonymous.clone()
                } else {
                    let anonymous = format!("keyword{}", counter);
                    map.insert(key, anonymous.clone());
                    counter += 1;
                    anonymous
                }
            })
            .to_string();
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anonymize_users() {
        let mut map = HashMap::new();
        let text = "Hey @john.doe and @jane_smith, check this out!";
        let result = anonymize_users(text, &mut map).unwrap();

        println!("Result: {}", result);
        assert!(result.contains("@user1"));
        assert!(result.contains("@user2"));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_slack_username_rules() {
        let mut map = HashMap::new();

        // Test lowercase requirement and various allowed characters
        let text = "Contact @alice, @bob_123, @test.user, and @dev-team";
        let result = anonymize_users(text, &mut map).unwrap();

        assert_eq!(map.len(), 4);
        assert!(result.contains("@user1"));
        assert!(result.contains("@user2"));
        assert!(result.contains("@user3"));
        assert!(result.contains("@user4"));
    }

    #[test]
    fn test_trailing_period_handling() {
        let mut map = HashMap::new();

        // Test that trailing periods are not part of the username
        let text = "See @john.doe. Also check @jane_smith.";
        let result = anonymize_users(text, &mut map).unwrap();

        // Should preserve the trailing period in output but not in mapping
        assert!(result.contains("@user1."));
        assert!(result.contains("@user2."));
        assert_eq!(map.len(), 2);

        // The keys should not have trailing periods
        assert!(map.contains_key("@john.doe"));
        assert!(map.contains_key("@jane_smith"));
    }

    #[test]
    fn test_anonymize_display_names() {
        let mut map = HashMap::new();
        let text = "**Jon Snow** Today at 3:17 PM\nHello Aria Stark and John Doe";
        let result = anonymize_display_names(text, &mut map).unwrap();

        assert!(result.contains("name1")); // Jon Snow
        assert!(result.contains("name2")); // Aria Stark  
        assert!(result.contains("name3")); // John Doe
        assert!(!result.contains("Jon Snow"));
        assert!(!result.contains("Aria Stark"));
        assert!(!result.contains("John Doe"));
    }

    #[test]
    fn test_display_name_pattern_requirements() {
        let mut map = HashMap::new();

        // Should match: exactly two words, both starting with uppercase
        let text = "Alice Smith and Bob Jones met with Carol White";
        let result = anonymize_display_names(text, &mut map).unwrap();

        assert_eq!(map.len(), 3);
        assert!(map.contains_key("Alice Smith"));
        assert!(map.contains_key("Bob Jones"));
        assert!(map.contains_key("Carol White"));
    }

    #[test]
    fn test_display_name_edge_cases() {
        let mut map = HashMap::new();

        // Should NOT match: single words, lowercase, three words
        let text = "john smith and Alice and Bob Smith Jones should not all match";
        let result = anonymize_display_names(text, &mut map).unwrap();

        // Only "Bob Smith" should match (exactly two words, both uppercase start)
        assert_eq!(map.len(), 1);
        assert!(map.contains_key("Bob Smith"));
        assert!(!map.contains_key("john smith")); // lowercase
        assert!(!map.contains_key("Alice")); // single word
        assert!(!map.contains_key("Smith Jones")); // part of three words
    }

    #[test]
    fn test_user_pattern_handles_both_formats() {
        let mut map = HashMap::new();

        // Test both @username and @Name Surname formats
        let text = "Contact @john.doe and @Aria Stark about the issue";
        let result = anonymize_users(text, &mut map).unwrap();

        assert_eq!(map.len(), 2);
        assert!(map.contains_key("@john.doe"));
        assert!(map.contains_key("@Aria Stark"));

        assert!(result.contains("@user1"));
        assert!(result.contains("@user2"));
        assert!(!result.contains("@john.doe"));
        assert!(!result.contains("@Aria Stark"));
    }

    #[test]
    fn test_slack_message_format() {
        let mut display_map = HashMap::new();
        let mut user_map = HashMap::new();

        let text = "**Jon Snow Jon Snow**  Today at 3:17 pm\n@Aria Stark глянь пліз до цього";

        // keep the same order as in anonymizer
        let result = anonymize_users(text, &mut user_map).unwrap();
        let result = anonymize_display_names(&result, &mut display_map).unwrap();

        assert!(result.contains("name1 name1")); // Jon Snow appears twice
        assert!(!result.contains("Jon Snow"));
        assert!(!result.contains("@Aria Stark"));
        assert!(result.contains("@user1"));

        assert_eq!(display_map.len(), 1);
        assert_eq!(user_map.len(), 1);
    }

    #[test]
    fn test_anonymize_channels() {
        let mut map = HashMap::new();
        let text = "Check #general and #random-thoughts";
        let result = anonymize_channels(text, &mut map).unwrap();

        assert!(result.contains("#ch1"));
        assert!(result.contains("#ch2"));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_anonymize_emails() {
        let mut map = HashMap::new();
        let text = "Contact john@company.com or support@client.org";
        let result = anonymize_emails(text, &mut map).unwrap();

        assert!(result.contains("user1@domain1.com"));
        assert!(result.contains("user2@domain2.com"));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_anonymize_urls() {
        let mut map = HashMap::new();
        let text = "Visit https://company.com/docs and http://client.org";
        let result = anonymize_urls(text, &mut map).unwrap();

        assert!(result.contains("https://example1.com/docs"));
        assert!(result.contains("https://example2.com"));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_anonymize_keywords() {
        let mut map = HashMap::new();
        let keywords = vec!["ProjectX".to_string(), "ClientABC".to_string()];
        let text = "ProjectX needs review and ClientABC approved it";
        let result = anonymize_keywords(text, &keywords, &mut map).unwrap();

        assert!(result.contains("keyword1"));
        assert!(result.contains("keyword2"));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_consistency() {
        let mut map = HashMap::new();
        let text1 = "Hey @john.doe";
        let text2 = "Hi @john.doe again";

        let result1 = anonymize_users(text1, &mut map).unwrap();
        let result2 = anonymize_users(text2, &mut map).unwrap();

        assert_eq!(result1, "Hey @user1");
        assert_eq!(result2, "Hi @user1 again");
        assert_eq!(map.len(), 1);
    }
}
