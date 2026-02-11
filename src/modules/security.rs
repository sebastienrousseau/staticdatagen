// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Security.txt Generation Module
//!
//! This module handles the creation and generation of security.txt files according to
//! RFC 9116 (<https://www.rfc-editor.org/rfc/rfc9116.html>). The security.txt file
//! helps security researchers report security vulnerabilities by providing standard
//! contact and policy information.

use crate::models::data::SecurityData;
use dtt::datetime::DateTime;
use log::debug;
use std::collections::HashMap;

/// Creates a SecurityData object from metadata.
///
/// Processes metadata to create a security.txt configuration, validating all fields
/// according to RFC 9116 requirements.
///
/// # Arguments
/// * `metadata` - A reference to a HashMap containing metadata key-value pairs
///
/// # Returns
/// * `SecurityData` - A struct containing the security.txt configuration
pub fn create_security_data(
    metadata: &HashMap<String, String>,
) -> SecurityData {
    SecurityData {
        contact: sanitize_urls(
            metadata
                .get("security_contact")
                .filter(|s| !s.is_empty())
                .map(|s| s.split(',').map(str::trim).collect())
                .unwrap_or_default(),
        ),
        expires: sanitize_expires(
            metadata
                .get("security_expires")
                .filter(|s| !s.is_empty())
                .unwrap_or(&String::new()),
        ),
        acknowledgments: sanitize_url(
            metadata
                .get("security_acknowledgments")
                .filter(|s| !s.is_empty())
                .unwrap_or(&String::new()),
        ),
        preferred_languages: sanitize_languages(
            metadata
                .get("security_languages")
                .filter(|s| !s.is_empty())
                .unwrap_or(&String::new()),
        ),
        canonical: sanitize_url(
            metadata
                .get("security_canonical")
                .filter(|s| !s.is_empty())
                .unwrap_or(&String::new()),
        ),
        policy: sanitize_url(
            metadata
                .get("security_policy")
                .filter(|s| !s.is_empty())
                .unwrap_or(&String::new()),
        ),
        hiring: sanitize_url(
            metadata
                .get("security_hiring")
                .filter(|s| !s.is_empty())
                .unwrap_or(&String::new()),
        ),
        encryption: sanitize_url(
            metadata
                .get("security_encryption")
                .filter(|s| !s.is_empty())
                .unwrap_or(&String::new()),
        ),
    }
}

/// Generates security.txt content.
///
/// Creates properly formatted security.txt content following RFC 9116 specifications.
///
/// # Arguments
/// * `data` - A reference to a SecurityData object containing the configuration
///
/// # Returns
/// * `String` - The generated security.txt content
pub fn generate_security_content(data: &SecurityData) -> String {
    if data.contact.is_empty() {
        debug!("Contact field is empty, no content generated.");
        return String::new();
    }

    let mut content = String::with_capacity(500);

    for contact in &data.contact {
        content.push_str(&format!("Contact: {}\n", contact));
    }

    if !data.expires.is_empty() {
        content.push_str(&format!("Expires: {}\n", data.expires));
    }

    // Add optional fields if present
    if !data.acknowledgments.is_empty() {
        content.push_str(&format!(
            "Acknowledgments: {}\n",
            data.acknowledgments
        ));
    }
    if !data.preferred_languages.is_empty() {
        content.push_str(&format!(
            "Preferred-Languages: {}\n",
            data.preferred_languages
        ));
    }
    if !data.canonical.is_empty() {
        content.push_str(&format!("Canonical: {}\n", data.canonical));
    }
    if !data.policy.is_empty() {
        content.push_str(&format!("Policy: {}\n", data.policy));
    }
    if !data.hiring.is_empty() {
        content.push_str(&format!("Hiring: {}\n", data.hiring));
    }
    if !data.encryption.is_empty() {
        content.push_str(&format!("Encryption: {}\n", data.encryption));
    }

    debug!("Generated security.txt content:\n{}", content);
    content
}

/// Sanitizes a list of URLs.
///
/// Validates each URL in the list according to RFC specifications.
fn sanitize_urls(urls: Vec<&str>) -> Vec<String> {
    urls.into_iter()
        .map(sanitize_url)
        .filter(|url| !url.is_empty())
        .collect()
}

/// Sanitizes and validates a URL.
///
/// Ensures URLs follow RFC specifications and contain no dangerous characters.
fn sanitize_url(url: &str) -> String {
    if url.is_empty() {
        return String::new();
    }

    // Allow plain text for acknowledgments
    if !url.contains(':') && !url.contains('<') && !url.contains('>') {
        return url.to_string();
    }

    // Validate standard URL schemes
    if !(url.starts_with("https://")
        || url.starts_with("http://")
        || url.starts_with("mailto:")
        || url.starts_with("tel:"))
    {
        return String::new();
    }

    url.to_string()
}

/// Sanitizes and validates the expiration date.
///
/// Ensures the date is in proper ISO 8601 format using DTT library.
// In security.rs, modify sanitize_expires:
// Sanitizes and validates the expiration date.
/// Handles multiple date formats and converts to RFC 3339.
fn sanitize_expires(date: &str) -> String {
    if date.is_empty() {
        return String::new();
    }

    debug!("Attempting to parse date: {}", date);

    // First try parsing as RFC 3339/ISO 8601
    let formatted = match DateTime::parse(date) {
        Ok(dt) => dt.format_rfc3339().unwrap_or_default(),
        Err(_) => {
            // If that fails, try converting from RFC 2822 format to ISO format
            // RFC 2822: "Tue, 20 Feb 2024 15:15:15 GMT"
            // Convert to: "2024-02-20T15:15:15Z"
            if let Some(iso_date) = convert_rfc2822_to_iso8601(date) {
                match DateTime::parse(&iso_date) {
                    Ok(dt) => dt.format_rfc3339().unwrap_or_default(),
                    Err(_) => String::new(),
                }
            } else {
                String::new()
            }
        }
    };

    debug!("Formatted date: {}", formatted);
    formatted
}

/// Converts an RFC 2822 formatted date to ISO 8601/RFC 3339 format
fn convert_rfc2822_to_iso8601(date: &str) -> Option<String> {
    // Example: "Tue, 20 Feb 2024 15:15:15 GMT" -> "2024-02-20T15:15:15Z"

    // Split the date parts
    let parts: Vec<&str> = date
        .split(", ")
        .nth(1)? // "20 Feb 2024 15:15:15 GMT"
        .split(' ')
        .collect();

    if parts.len() < 5 {
        return None;
    }

    // Parse day
    let day = parts.first()?;

    // Convert month
    let month = match *parts.get(1)? {
        "Jan" => "01",
        "Feb" => "02",
        "Mar" => "03",
        "Apr" => "04",
        "May" => "05",
        "Jun" => "06",
        "Jul" => "07",
        "Aug" => "08",
        "Sep" => "09",
        "Oct" => "10",
        "Nov" => "11",
        "Dec" => "12",
        _ => return None,
    };

    // Get year and time
    let year = parts.get(2)?;
    let time = parts.get(3)?;

    // Format as ISO 8601/RFC 3339
    Some(format!(
        "{}-{}-{:02}T{}Z",
        year,
        month,
        day.parse::<u8>().ok()?,
        time
    ))
}

/// Sanitizes and validates language tags.
///
/// Ensures language tags follow IETF language tag format.
fn sanitize_languages(languages: &str) -> String {
    if languages.is_empty() {
        return String::new();
    }

    let valid_languages: Vec<String> = languages
        .split(',')
        .map(str::trim)
        .filter(|lang| {
            // Basic IETF language tag validation
            !lang.is_empty()
                && lang
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-')
        })
        .map(str::to_string)
        .collect();

    if valid_languages.is_empty() {
        String::new()
    } else {
        valid_languages.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_security_data() {
        let mut metadata = HashMap::new();
        let _ = metadata.insert(
            "security_contact".to_string(),
            "https://example.com/security".to_string(),
        );
        let _ = metadata.insert(
            "security_expires".to_string(),
            "2024-12-31T23:59:59Z".to_string(),
        );

        let data = create_security_data(&metadata);
        assert_eq!(data.contact[0], "https://example.com/security");
        assert!(data.expires.contains("2024-12-31"));
    }

    #[test]
    fn test_generate_security_content() {
        let data = SecurityData {
            contact: vec!["https://example.com/security".to_string()],
            expires: "2024-12-31T23:59:59Z".to_string(),
            acknowledgments: "https://example.com/thanks".to_string(),
            preferred_languages: "en, fr".to_string(),
            canonical: "https://example.com/.well-known/security.txt"
                .to_string(),
            policy: String::new(),
            hiring: String::new(),
            encryption: String::new(),
        };

        let content = generate_security_content(&data);
        assert!(
            content.contains("Contact: https://example.com/security")
        );
        assert!(content.contains("Expires: 2024-12-31"));
        assert!(content.contains("Preferred-Languages: en, fr"));
    }

    #[test]
    fn test_sanitize_url() {
        assert_eq!(
            sanitize_url("https://example.com/security"),
            "https://example.com/security"
        );
        assert_eq!(
            sanitize_url("mailto:security@example.com"),
            "mailto:security@example.com"
        );
        assert!(sanitize_url("ftp://example.com").is_empty());
        assert!(sanitize_url("javascript:alert(1)").is_empty());
    }

    #[test]
    fn test_sanitize_expires() {
        // Test RFC 3339 format
        assert!(!sanitize_expires("2024-12-31T23:59:59Z").is_empty());

        // Test RFC 2822 format
        assert!(!sanitize_expires("Tue, 20 Feb 2024 15:15:15 GMT")
            .is_empty());

        // Test invalid dates
        assert!(sanitize_expires("invalid-date").is_empty());
        assert!(sanitize_expires("").is_empty());
    }

    #[test]
    fn test_convert_rfc2822_to_iso8601() {
        // Test valid RFC 2822 date
        assert_eq!(
            convert_rfc2822_to_iso8601("Tue, 20 Feb 2024 15:15:15 GMT"),
            Some("2024-02-20T15:15:15Z".to_string())
        );

        // Test invalid dates
        assert_eq!(convert_rfc2822_to_iso8601("invalid date"), None);
        assert_eq!(convert_rfc2822_to_iso8601(""), None);
    }

    #[test]
    fn test_rfc2822_dates() {
        let test_cases = vec![
            ("Mon, 15 Jan 2024 10:30:00 GMT", true),
            ("Tue, 20 Feb 2024 15:15:15 GMT", true),
            ("Wed, 31 Dec 2024 23:59:59 GMT", true),
            ("Invalid Date", false),
        ];

        for (input, should_succeed) in test_cases {
            let result = sanitize_expires(input);
            assert_eq!(
                !result.is_empty(),
                should_succeed,
                "Failed for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_sanitize_languages() {
        assert_eq!(sanitize_languages("en, fr, de"), "en, fr, de");
        assert_eq!(sanitize_languages("en-US, fr-FR"), "en-US, fr-FR");
        assert!(sanitize_languages("<script>").is_empty());
    }

    #[test]
    fn test_multiple_contacts() {
        let mut metadata = HashMap::new();
        let _ = metadata.insert(
            "security_contact".to_string(),
            "https://example.com/security, mailto:security@example.com"
                .to_string(),
        );

        let data = create_security_data(&metadata);
        assert_eq!(data.contact.len(), 2);
        assert!(data
            .contact
            .contains(&"https://example.com/security".to_string()));
        assert!(data
            .contact
            .contains(&"mailto:security@example.com".to_string()));
    }

    #[test]
    fn test_empty_security_data() {
        let data = create_security_data(&HashMap::new());
        assert!(data.contact.is_empty());
        assert!(data.expires.is_empty());
    }

    #[test]
    fn test_generate_security_content_empty_contact() {
        // Test line 91-93: empty contact returns empty string
        let data = SecurityData {
            contact: vec![],
            expires: "2024-12-31T23:59:59Z".to_string(),
            acknowledgments: String::new(),
            preferred_languages: String::new(),
            canonical: String::new(),
            policy: String::new(),
            hiring: String::new(),
            encryption: String::new(),
        };

        let content = generate_security_content(&data);
        assert!(content.is_empty());
    }

    #[test]
    fn test_generate_security_content_all_fields() {
        // Test lines 122-129: policy, hiring, encryption fields
        let data = SecurityData {
            contact: vec!["mailto:security@example.com".to_string()],
            expires: "2024-12-31T23:59:59Z".to_string(),
            acknowledgments: "https://example.com/thanks".to_string(),
            preferred_languages: "en".to_string(),
            canonical: "https://example.com/.well-known/security.txt".to_string(),
            policy: "https://example.com/security-policy".to_string(),
            hiring: "https://example.com/jobs".to_string(),
            encryption: "https://example.com/pgp-key.txt".to_string(),
        };

        let content = generate_security_content(&data);
        assert!(content.contains("Contact: mailto:security@example.com"));
        assert!(content.contains("Policy: https://example.com/security-policy"));
        assert!(content.contains("Hiring: https://example.com/jobs"));
        assert!(content.contains("Encryption: https://example.com/pgp-key.txt"));
    }

    #[test]
    fn test_sanitize_url_plain_text() {
        // Test line 155-156: plain text without colons
        assert_eq!(sanitize_url("Security Team"), "Security Team");
        assert_eq!(sanitize_url("John Doe"), "John Doe");
    }

    #[test]
    fn test_sanitize_url_with_angle_brackets() {
        // Test that URLs with angle brackets are rejected
        assert!(sanitize_url("text<script>").is_empty());
        assert!(sanitize_url("text>more").is_empty());
    }

    #[test]
    fn test_convert_rfc2822_invalid_month() {
        // Test line 238: unknown month returns None
        assert_eq!(
            convert_rfc2822_to_iso8601("Tue, 20 Xyz 2024 15:15:15 GMT"),
            None
        );
    }

    #[test]
    fn test_convert_rfc2822_insufficient_parts() {
        // Test line 217-218: fewer than 5 parts returns None
        assert_eq!(convert_rfc2822_to_iso8601("Tue, 20 Feb 2024"), None);
        assert_eq!(convert_rfc2822_to_iso8601("Tue, 20 Feb"), None);
    }

    #[test]
    fn test_convert_rfc2822_all_months() {
        // Test all month conversions (lines 226-237)
        let months = [
            ("Tue, 15 Jan 2024 10:00:00 GMT", "01"),
            ("Tue, 15 Feb 2024 10:00:00 GMT", "02"),
            ("Tue, 15 Mar 2024 10:00:00 GMT", "03"),
            ("Tue, 15 Apr 2024 10:00:00 GMT", "04"),
            ("Tue, 15 May 2024 10:00:00 GMT", "05"),
            ("Tue, 15 Jun 2024 10:00:00 GMT", "06"),
            ("Tue, 15 Jul 2024 10:00:00 GMT", "07"),
            ("Tue, 15 Aug 2024 10:00:00 GMT", "08"),
            ("Tue, 15 Sep 2024 10:00:00 GMT", "09"),
            ("Tue, 15 Oct 2024 10:00:00 GMT", "10"),
            ("Tue, 15 Nov 2024 10:00:00 GMT", "11"),
            ("Tue, 15 Dec 2024 10:00:00 GMT", "12"),
        ];

        for (input, expected_month) in months {
            let result = convert_rfc2822_to_iso8601(input);
            assert!(result.is_some(), "Failed to parse: {}", input);
            assert!(
                result.as_ref().unwrap().contains(&format!("-{}-", expected_month)),
                "Expected month {} in result {:?} for input {}",
                expected_month,
                result,
                input
            );
        }
    }

    #[test]
    fn test_sanitize_urls_filters_empty() {
        // Test that sanitize_urls filters out empty results
        let urls = vec!["https://example.com", "javascript:alert(1)", "mailto:test@example.com"];
        let result = sanitize_urls(urls);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"https://example.com".to_string()));
        assert!(result.contains(&"mailto:test@example.com".to_string()));
    }

    #[test]
    fn test_sanitize_languages_empty_tags() {
        // Test that empty language tags are filtered
        assert_eq!(sanitize_languages("en, , fr"), "en, fr");
        assert_eq!(sanitize_languages(",  ,  "), "");
    }

    #[test]
    fn test_sanitize_url_tel_scheme() {
        // Test tel: scheme is accepted
        assert_eq!(sanitize_url("tel:+1234567890"), "tel:+1234567890");
    }

    #[test]
    fn test_sanitize_url_http_scheme() {
        // Test http: scheme is accepted
        assert_eq!(sanitize_url("http://example.com"), "http://example.com");
    }

    #[test]
    fn test_create_security_data_all_fields() {
        let mut metadata = HashMap::new();
        let _ = metadata.insert("security_contact".to_string(), "mailto:sec@example.com".to_string());
        let _ = metadata.insert("security_expires".to_string(), "2024-12-31T23:59:59Z".to_string());
        let _ = metadata.insert("security_acknowledgments".to_string(), "https://example.com/thanks".to_string());
        let _ = metadata.insert("security_languages".to_string(), "en, fr".to_string());
        let _ = metadata.insert("security_canonical".to_string(), "https://example.com/security.txt".to_string());
        let _ = metadata.insert("security_policy".to_string(), "https://example.com/policy".to_string());
        let _ = metadata.insert("security_hiring".to_string(), "https://example.com/jobs".to_string());
        let _ = metadata.insert("security_encryption".to_string(), "https://example.com/pgp".to_string());

        let data = create_security_data(&metadata);
        assert!(!data.contact.is_empty());
        assert!(!data.expires.is_empty());
        assert!(!data.acknowledgments.is_empty());
        assert!(!data.preferred_languages.is_empty());
        assert!(!data.canonical.is_empty());
        assert!(!data.policy.is_empty());
        assert!(!data.hiring.is_empty());
        assert!(!data.encryption.is_empty());
    }
}
