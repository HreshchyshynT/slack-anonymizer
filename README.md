# Slack Anonymizer

A Rust CLI tool that anonymizes Slack text by replacing sensitive information like usernames, display names, channel names, email addresses, URLs, and custom keywords with anonymous placeholders while maintaining consistency.

It doesn't have any fancy algorithms under the hood, only straightforward regular expressions. 

Mostly vibe coded.

Created for the following flow: 

1. Select and copy thread or messages in slack
2. use `pbpaste | slack-anonymizer | pbcopy` in your terminal
3. Paste filtered text into your llm to create jira ticket description

## Features

- **User mentions**: `@username` → `@user1`, `@user2`, etc.
- **Display names**: `Jon Snow` → `name1`, `Aria Stark` → `name2`, etc.
- **Channel references**: `#channel-name` → `#ch1`, `#ch2`, etc.
- **Email addresses**: `user@domain.com` → `user1@domain1.com`, etc.
- **URLs**: `https://company.com` → `https://example1.com` (optional)
- **Custom keywords**: Replace specified terms with `keyword1`, `keyword2`, etc.
- **Legend**: Optional mapping of original → anonymous values

## Sample Input/Output

### Slack Message Format
**Input:**
```
**Jon Snow Jon Snow**  Today at 3:17 pm
@Aria Stark глянь пліз до цього
Contact support@company.com for ProjectX details.
```

**Output (with `--urls --replace "ProjectX" --legend`):**
```
**name1 name1**  Today at 3:17 pm
@user1 глянь пліз до цього
Contact user1@domain1.com for keyword1 details.

=== ANONYMIZATION LEGEND ===
@Aria Stark → @user1
Jon Snow → name1
projectx → keyword1
support@company.com → user1@domain1.com
```

### Smart Name Matching
**Input:**
```
**Jon Snow** mentioned something
Later, @Jon Snow replied to the thread
```

**Output:**
```
**user1** mentioned something  
Later, @user1 replied to the thread
```

## Anonymization Rules

### Processing Order
1. User mentions (`@username`, `@Name Surname`) - processed first to establish username mappings
2. Display names (`Jon Snow`, `Aria Stark`) - can reuse username mappings for consistency
3. Channel references (`#channel-name`)
4. Email addresses
5. URLs (if `--urls` flag is used)
6. Custom keywords (if `--replace` is specified)


## License

This project is licensed under the MIT License - see the LICENSE file for details.

