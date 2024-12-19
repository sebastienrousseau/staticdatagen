// Copyright © 2024 StaticDataGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # CNAME Record Generation Module
//!
//! This module provides functionality for generating CNAME (Canonical Name) records
//! following DNS standards as defined in RFC 1035. It includes domain validation,
//! Time-To-Live (TTL) handling, and customizable record formats.
//!
//! ## Features
//! - **Domain Validation**: Ensure compliance with DNS standards.
//! - **Internationalized Domain Names (IDN)**: Support for Unicode domains via Punycode conversion.
//! - **Custom Formats**: Enable user-defined CNAME record formats.
//! - **Batch Processing**: Generate multiple records efficiently using parallel processing.
//!
//! ## Example Usage
//! ```rust
//! use staticdatagen::generators::cname::{CnameConfig, CnameGenerator};
//!
//! let config = CnameConfig::new("example.com", Some(7200), None).unwrap();
//! let generator = CnameGenerator::new(config);
//! let cname_record = generator.generate();
//!
//! assert!(cname_record.contains("example.com"));
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// ## Errors in CNAME Record Processing
///
/// Represents various errors that may occur during CNAME record generation and validation.
#[derive(Debug, Error)]
pub enum CnameError {
    /// The domain name is empty or missing.
    #[error("Domain name cannot be empty.")]
    EmptyDomain,
    /// The domain contains invalid characters.
    #[error("Domain contains invalid characters: {0}")]
    InvalidCharacters(String),
    /// A domain label exceeds 63 characters.
    #[error(
        "Domain label exceeds maximum length of 63 characters: {0}"
    )]
    LabelTooLong(String),
    /// The domain name format is invalid.
    #[error("Invalid domain format: {0}")]
    MalformedDomain(String),
    /// A domain label starts or ends with a hyphen.
    #[error("Domain labels cannot start or end with hyphens: {0}")]
    InvalidHyphenUsage(String),
    /// The TTL value provided is invalid.
    #[error("Invalid TTL value: {0}")]
    InvalidTtl(String),
    /// The total domain length exceeds 255 characters.
    #[error("Total domain length exceeds 255 characters: {0}")]
    ExcessiveDomainLength(String),
}

/// ## CNAME Configuration
///
/// Represents the configuration needed to generate a CNAME record, including validation
/// to ensure compliance with DNS standards.
#[derive(
    Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize,
)]
pub struct CnameConfig {
    /// The domain name for the CNAME record.
    pub domain: String,
    /// The Time-To-Live (TTL) value for the CNAME record.
    pub ttl: u32,
    /// An optional custom format for the CNAME record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl CnameConfig {
    /// The default TTL value for CNAME records.
    const DEFAULT_TTL: u32 = 3600;

    /// Creates a new validated CNAME configuration.
    ///
    /// # Arguments
    ///
    /// - `domain`: The domain name to use for the CNAME record.
    /// - `ttl`: The TTL value (defaults to `3600` seconds if `None`).
    /// - `format`: An optional custom record format.
    ///
    /// # Returns
    ///
    /// A `Result` containing the validated `CnameConfig` or a `CnameError`.
    ///
    /// # Example
    /// ```rust
    /// use staticdatagen::generators::cname::CnameConfig;
    ///
    /// let config = CnameConfig::new("example.com", Some(3600), None).unwrap();
    /// assert_eq!(config.ttl, 3600);
    /// ```
    pub fn new(
        domain: impl Into<String>,
        ttl: Option<u32>,
        format: Option<String>,
    ) -> Result<Self, CnameError> {
        let domain =
            Self::validate_and_normalise_domain(domain.into())?;
        let ttl = ttl.unwrap_or(Self::DEFAULT_TTL);

        if ttl == 0 {
            return Err(CnameError::InvalidTtl(
                "TTL must be greater than 0.".to_string(),
            ));
        }

        Ok(Self {
            domain,
            ttl,
            format,
        })
    }

    /// Validates and normalises a domain name.
    ///
    /// Handles validation and Punycode conversion for internationalized domains.
    ///
    /// # Arguments
    ///
    /// - `domain`: The domain name to validate and normalize.
    ///
    /// # Returns
    ///
    /// A `Result` containing the validated and normalized domain name or a `CnameError`.
    fn validate_and_normalise_domain(
        domain: String,
    ) -> Result<String, CnameError> {
        // Check for leading or trailing whitespace
        if domain.trim() != domain {
            return Err(CnameError::InvalidCharacters(
                "Domain contains leading or trailing whitespace."
                    .to_string(),
            ));
        }

        let domain = domain.trim(); // Normalize whitespace after the check

        if domain.is_empty() {
            return Err(CnameError::EmptyDomain);
        }

        // Convert IDNs to ASCII (Punycode)
        let ascii_domain =
            idna::domain_to_ascii(domain).map_err(|_| {
                CnameError::InvalidCharacters(format!(
                    "Invalid domain format: {domain}"
                ))
            })?;

        Self::validate_domain(&ascii_domain)?;
        Ok(ascii_domain)
    }

    /// Validates a domain name for compliance with DNS standards.
    ///
    /// # Arguments
    ///
    /// - `domain`: The domain name to validate.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or a `CnameError` if validation fails.
    fn validate_domain(domain: &str) -> Result<(), CnameError> {
        if domain.len() > 255 {
            return Err(CnameError::ExcessiveDomainLength(
                domain.to_string(),
            ));
        }

        let labels: Vec<&str> = domain.split('.').collect();
        if labels.len() < 2 {
            return Err(CnameError::MalformedDomain(
            "Domain must have at least two parts (e.g., example.com).".to_string(),
        ));
        }

        for label in labels {
            if label.is_empty() {
                return Err(CnameError::MalformedDomain(
                    "Empty label in domain name.".to_string(),
                ));
            }
            if label.len() > 63 {
                return Err(CnameError::LabelTooLong(
                    label.to_string(),
                ));
            }
            if label.starts_with('-') || label.ends_with('-') {
                return Err(CnameError::InvalidHyphenUsage(
                    label.to_string(),
                ));
            }
            if !label
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-')
            {
                return Err(CnameError::InvalidCharacters(
                    label.to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Generates a formatted CNAME record using the configuration.
    ///
    /// # Returns
    ///
    /// A formatted CNAME record as a string.
    pub fn generate_custom(&self) -> String {
        if let Some(ref fmt) = self.format {
            fmt.replace("{domain}", &self.domain)
                .replace("{ttl}", &self.ttl.to_string())
        } else {
            format!(
                "{domain} {ttl} IN CNAME www.{domain}",
                domain = self.domain,
                ttl = self.ttl
            )
        }
    }
}

/// ## CNAME Generator
///
/// Facilitates the generation of CNAME records using the provided configuration.
#[derive(Debug)]
pub struct CnameGenerator {
    /// The configuration for the CNAME record.
    pub config: CnameConfig,
}

impl CnameGenerator {
    /// Creates a new generator with the provided configuration.
    ///
    /// # Arguments
    ///
    /// - `config`: The `CnameConfig` containing the validated inputs.
    ///
    /// # Returns
    ///
    /// A new `CnameGenerator` instance.
    pub fn new(config: CnameConfig) -> Self {
        Self { config }
    }

    /// Generates the CNAME record as a string.
    ///
    /// # Returns
    ///
    /// A formatted CNAME record.
    pub fn generate(&self) -> String {
        self.config.generate_custom()
    }

    /// Exports the generated CNAME record to a file.
    ///
    /// # Arguments
    ///
    /// - `path`: The path to the file where the record will be written.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn export_to_file(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.generate())
    }
    /// Generates multiple CNAME records in batch using parallel processing.
    ///
    /// # Arguments
    ///
    /// - `configs`: A vector of `CnameConfig` instances.
    ///
    /// # Returns
    ///
    /// A vector of results, where each result is either the generated CNAME record or a `CnameError`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use staticdatagen::generators::cname::{CnameConfig, CnameGenerator};
    ///
    /// let configs = vec![
    ///     CnameConfig::new("example.com", Some(7200), None).unwrap(),
    ///     CnameConfig::new("sub.example.com", Some(3600), None).unwrap(),
    /// ];
    ///
    /// let records = CnameGenerator::batch_generate(configs);
    /// assert_eq!(records.len(), 2);
    /// ```
    pub fn batch_generate(
        configs: Vec<CnameConfig>,
    ) -> Vec<Result<String, CnameError>> {
        use rayon::prelude::*;

        configs
            .into_par_iter() // Use parallel iterator for efficiency
            .map(|config| {
                let generator = CnameGenerator::new(config);
                Ok(generator.generate())
            })
            .collect()
    }

    /// Exports multiple CNAME records to a file in batch using parallel processing.
    ///
    /// # Arguments
    ///
    /// - `configs`: A vector of `CnameConfig` instances.
    /// - `path`: The path to the file where the records will be saved.
    /// - `delimiter`: A string delimiter used to separate the records in the file.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure. If any CNAME generation fails, the function will return the first encountered error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use staticdatagen::generators::cname::{CnameConfig, CnameGenerator};
    /// use std::fs;
    ///
    /// let configs = vec![
    ///     CnameConfig::new("example.com", Some(7200), None).unwrap(),
    ///     CnameConfig::new("sub.example.com", Some(3600), None).unwrap(),
    /// ];
    ///
    /// let file_path = "CNAME";
    ///
    /// // Export the batch of CNAME records to a file
    /// let result = CnameGenerator::export_batch_to_file(configs, file_path, "\n");
    /// assert!(result.is_ok(), "Failed to export batch to file");
    ///
    /// // Verify file content (optional)
    /// let content = fs::read_to_string(file_path).unwrap();
    /// assert!(content.contains("example.com"), "File content missing expected record");
    ///
    /// // Cleanup: Remove the file after the test
    /// fs::remove_file(file_path).expect("Failed to remove test file");
    /// ```
    pub fn export_batch_to_file(
        configs: Vec<CnameConfig>,
        path: &str,
        delimiter: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let results = Self::batch_generate(configs);

        // Separate successful results from errors
        let (successes, errors): (Vec<_>, Vec<_>) =
            results.into_iter().partition(Result::is_ok);

        // If there are errors, return the first encountered error
        if let Some(Err(err)) = errors.into_iter().next() {
            return Err(Box::new(err));
        }

        // Concatenate all successful records with the specified delimiter
        let content = successes
            .into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<_>>()
            .join(delimiter);

        // Write the combined content to the specified file
        std::fs::write(path, content)?;

        Ok(())
    }

    /// Creates a CNAME record from metadata provided as a key-value map.
    ///
    /// # Arguments
    ///
    /// - `metadata`: A `HashMap` containing the metadata keys and values. The `cname` key is mandatory.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated CNAME record or a `CnameError`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use staticdatagen::generators::cname::CnameGenerator;
    ///
    /// let mut metadata = HashMap::new();
    /// metadata.insert("cname".to_string(), "example.com".to_string());
    /// metadata.insert("ttl".to_string(), "7200".to_string());
    ///
    /// let content = CnameGenerator::from_metadata(&metadata).unwrap();
    /// assert!(content.contains("example.com"));
    /// ```
    pub fn from_metadata(
        metadata: &HashMap<String, String>,
    ) -> Result<String, CnameError> {
        let domain =
            metadata.get("cname").ok_or(CnameError::EmptyDomain)?;

        let ttl = metadata
            .get("ttl")
            .map(|value| value.parse::<u32>())
            .transpose()
            .map_err(|_| {
                CnameError::InvalidTtl("Invalid TTL value.".into())
            })?
            .unwrap_or(3600); // Default to 3600 seconds if no TTL is provided

        let format = metadata.get("format").cloned();

        let config = CnameConfig::new(domain, Some(ttl), format)?;
        let generator = CnameGenerator::new(config);

        Ok(generator.generate())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_valid_domain_names() {
        let valid_domains = [
            "example.com",
            "sub.example.com",
            "my-site.example.com",
            "example.co.uk",
        ];

        for domain in &valid_domains {
            assert!(
                CnameConfig::new(*domain, None, None).is_ok(),
                "Domain should be valid: {}",
                domain
            );
        }
    }

    #[test]
    fn test_invalid_domain_names() {
        let invalid_domains = [
            "",              // Empty
            "example",       // No TLD
            "-example.com",  // Starts with hyphen
            "example-.com",  // Ends with hyphen
            "exam ple.com",  // Contains space
            "example..com",  // Double dots
            &"a".repeat(64), // Label too long
            "exam@ple.com",  // Invalid character
        ];

        for domain in &invalid_domains {
            assert!(
                CnameConfig::new(*domain, None, None).is_err(),
                "Domain should be invalid: {}",
                domain
            );
        }
    }

    #[test]
    fn test_cname_generation() {
        let config =
            CnameConfig::new("example.com", Some(7200), None).unwrap();
        let generator = CnameGenerator::new(config);
        let content = generator.generate();

        assert_eq!(
            content,
            "example.com 7200 IN CNAME www.example.com"
        );
    }

    #[test]
    fn test_cname_with_different_ttl() {
        let config =
            CnameConfig::new("example.com", Some(1800), None).unwrap();
        let generator = CnameGenerator::new(config);
        let content = generator.generate();

        assert_eq!(
            content,
            "example.com 1800 IN CNAME www.example.com"
        );
    }

    #[test]
    fn test_default_ttl() {
        let config =
            CnameConfig::new("example.com", None, None).unwrap();
        assert_eq!(config.ttl, 3600);
    }

    #[test]
    fn test_cname_error_empty_domain() {
        let result = CnameConfig::new("", None, None);
        assert!(matches!(result, Err(CnameError::EmptyDomain)));
    }

    #[test]
    fn test_cname_error_invalid_characters() {
        let result = CnameConfig::new("exam@ple.com", None, None);
        assert!(matches!(
            result,
            Err(CnameError::InvalidCharacters(label)) if label == "exam@ple"
        ));
    }

    #[test]
    fn test_cname_error_label_too_long() {
        let long_label = "a".repeat(64);
        let domain = format!("{}.com", long_label);
        let result = CnameConfig::new(&domain, None, None);
        assert!(matches!(
            result,
            Err(CnameError::LabelTooLong(label)) if label == long_label
        ));
    }

    #[test]
    fn test_cname_error_malformed_domain() {
        let result = CnameConfig::new("example..com", None, None);
        assert!(matches!(
            result,
            Err(CnameError::MalformedDomain(message)) if message.contains("Empty label")
        ));
    }

    #[test]
    fn test_cname_error_invalid_hyphen_usage() {
        let result = CnameConfig::new("-example.com", None, None);
        assert!(matches!(
            result,
            Err(CnameError::InvalidHyphenUsage(label)) if label == "-example"
        ));
    }

    #[test]
    fn test_invalid_ttl() {
        let result = CnameConfig::new("example.com", Some(0), None);
        assert!(
            matches!(result, Err(CnameError::InvalidTtl(message)) if message.contains("TTL must be greater than 0"))
        );
    }

    #[test]
    fn test_empty_metadata() {
        let metadata: HashMap<String, String> = HashMap::new();
        let result = CnameGenerator::from_metadata(&metadata);
        assert!(matches!(result, Err(CnameError::EmptyDomain)));
    }

    #[test]
    fn test_invalid_metadata_domain() {
        let mut metadata = HashMap::new();
        _ = metadata
            .insert("cname".to_string(), "invalid_domain".to_string());
        let result = CnameGenerator::from_metadata(&metadata);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_metadata() {
        let mut metadata = HashMap::new();
        _ = metadata
            .insert("cname".to_string(), "example.com".to_string());
        _ = metadata.insert("ttl".to_string(), "3600".to_string()); // Add TTL to the metadata
        let result = CnameGenerator::from_metadata(&metadata).unwrap();

        assert_eq!(result, "example.com 3600 IN CNAME www.example.com");
    }

    #[test]
    fn test_min_label_length() {
        let domain = "a.b";
        let config = CnameConfig::new(domain, None, None);
        assert!(config.is_ok(), "Domain should be valid: {}", domain);
    }

    #[test]
    fn test_max_domain_length() {
        let max_label = "a".repeat(63);
        let domain = format!("{}.{}.{}", max_label, max_label, "a.com");
        let config = CnameConfig::new(&domain, None, None);
        assert!(config.is_ok(), "Domain should be valid: {}", domain);
    }

    #[test]
    fn test_max_ttl() {
        let config =
            CnameConfig::new("example.com", Some(u32::MAX), None)
                .unwrap();
        assert_eq!(config.ttl, u32::MAX);
    }

    #[test]
    fn test_invalid_domain_with_valid_ttl() {
        let result = CnameConfig::new("example..com", Some(7200), None);
        assert!(result.is_err(), "Domain should be invalid");
    }

    #[test]
    fn test_metadata_missing_cname_key() {
        let mut metadata = HashMap::new();
        _ = metadata
            .insert("other".to_string(), "example.com".to_string());
        let result = CnameGenerator::from_metadata(&metadata);
        assert!(matches!(result, Err(CnameError::EmptyDomain)));
    }

    #[test]
    fn test_case_sensitive_metadata_key() {
        let mut metadata = HashMap::new();
        _ = metadata
            .insert("CNAME".to_string(), "example.com".to_string());
        let result = CnameGenerator::from_metadata(&metadata);
        assert!(matches!(result, Err(CnameError::EmptyDomain)));
    }

    #[test]
    fn test_whitespace_in_domain() {
        let result = CnameConfig::new(" example.com ", None, None);
        assert!(
        matches!(result, Err(CnameError::InvalidCharacters(_))),
        "Expected InvalidCharacters error for domain with leading/trailing whitespace."
    );
    }

    #[test]
    fn test_generate_exact_format() {
        let config =
            CnameConfig::new("example.com", Some(7200), None).unwrap();
        let generator = CnameGenerator::new(config);
        let content = generator.generate();

        assert_eq!(
            content,
            "example.com 7200 IN CNAME www.example.com"
        );
    }

    #[test]
    fn test_label_length_at_limit() {
        let label = "a".repeat(63);
        let domain = format!("{}.com", label);
        let config = CnameConfig::new(&domain, None, None);
        assert!(config.is_ok(), "Domain should be valid: {}", domain);
    }

    #[test]
    fn test_label_length_exceeding_limit() {
        let label = "a".repeat(64);
        let domain = format!("{}.com", label);
        let result = CnameConfig::new(&domain, None, None);
        assert!(matches!(result, Err(CnameError::LabelTooLong(_))));
    }

    #[test]
    fn test_debug_output() {
        let config =
            CnameConfig::new("example.com", Some(7200), None).unwrap();
        let debug_output = format!("{:?}", config);
        assert!(debug_output.contains("example.com"));
        assert!(debug_output.contains("7200"));
    }

    #[test]
    fn test_parallel_batch_generation() {
        let configs = (0..100_000)
            .map(|i| {
                CnameConfig::new(
                    format!("example{}.com", i),
                    Some(3600),
                    None,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();

        let start = std::time::Instant::now();
        let records = CnameGenerator::batch_generate(configs);

        // Count successes and failures
        let (successes, failures): (Vec<_>, Vec<_>) =
            records.into_iter().partition(Result::is_ok);

        let duration = start.elapsed();

        println!(
            "Generated {} records successfully, {} failed, in {:?}",
            successes.len(),
            failures.len(),
            duration
        );

        assert_eq!(successes.len(), 100_000);
        assert!(failures.is_empty(), "There should be no failures");
    }

    #[test]
    fn test_large_domain() {
        let domain =
            format!("{}.{}.com", "a".repeat(63), "b".repeat(63));
        let config = CnameConfig::new(&domain, Some(3600), None);
        assert!(config.is_ok(), "Large domain should be valid");
    }

    // Test: Create CnameConfig with maximum valid domain length
    #[test]
    fn test_cname_config_max_domain_length() {
        let max_label = "a".repeat(63);
        let domain = format!("{}.{}.{}", max_label, max_label, "com");
        let config = CnameConfig::new(domain.clone(), Some(3600), None);
        assert!(config.is_ok(), "Domain should be valid: {}", domain);
    }

    // Test: Create CnameConfig with maximum TTL value
    #[test]
    fn test_cname_config_max_ttl() {
        let config =
            CnameConfig::new("example.com", Some(u32::MAX), None)
                .unwrap();
        assert_eq!(config.ttl, u32::MAX);
    }

    // Test: Export single CNAME record to file
    #[test]
    fn test_export_to_file() {
        let config =
            CnameConfig::new("example.com", Some(7200), None).unwrap();
        let generator = CnameGenerator::new(config);

        let file_path = "test_cname.txt";

        // Export the file
        generator.export_to_file(file_path).unwrap();

        // Verify file content
        let content = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(
            content,
            "example.com 7200 IN CNAME www.example.com"
        );

        // Cleanup after test verification
        std::fs::remove_file(file_path).unwrap();
        println!("🗑️ File '{}' removed after the test.", file_path);
    }

    // Test: Export batch CNAME records with errors
    #[test]
    fn test_export_batch_to_file_with_errors() {
        // Create a list of configs, including one that is invalid
        let configs = vec![
            CnameConfig::new("example.com", Some(3600), None), // Valid
            CnameConfig::new("invalid_domain", Some(3600), None), // Invalid
        ];

        // Filter out invalid configurations
        let valid_configs: Vec<CnameConfig> = configs
            .into_iter()
            .filter_map(Result::ok) // Keep only valid configurations
            .collect();

        let file_path = "test_batch_cname_with_errors.txt";
        let result = CnameGenerator::export_batch_to_file(
            valid_configs,
            file_path,
            "\n",
        );

        // Assert the file export was successful
        assert!(result.is_ok());

        // Read and verify the content of the exported file
        let content = std::fs::read_to_string(file_path).unwrap();
        assert!(content
            .contains("example.com 3600 IN CNAME www.example.com"));

        // Cleanup
        std::fs::remove_file(file_path).unwrap();
    }

    // Test: Parallel batch generation with large number of records
    #[test]
    fn test_parallel_batch_large_input() {
        let configs = (0..1_000)
            .map(|i| {
                CnameConfig::new(
                    format!("example{}.com", i),
                    Some(3600),
                    None,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();

        let results = CnameGenerator::batch_generate(configs);

        let successes: Vec<_> =
            results.into_iter().filter_map(Result::ok).collect();

        assert_eq!(successes.len(), 1_000);
    }

    // Test: Custom format generation
    #[test]
    fn test_custom_format_generation() {
        let config = CnameConfig::new(
            "example.com",
            Some(3600),
            Some("{domain} {ttl} IN ALIAS custom.{domain}".to_string()),
        )
        .unwrap();

        let generator = CnameGenerator::new(config);
        let record = generator.generate();

        assert_eq!(
            record,
            "example.com 3600 IN ALIAS custom.example.com"
        );
    }

    // Test: Batch generation with delimiter in output
    #[test]
    fn test_batch_generate_with_delimiter() {
        let configs = vec![
            CnameConfig::new("example.com", Some(3600), None).unwrap(),
            CnameConfig::new("sub.example.com", Some(3600), None)
                .unwrap(),
        ];

        let file_path = "test_delimited_cname.txt";
        let result = CnameGenerator::export_batch_to_file(
            configs, file_path, "---\n",
        );

        assert!(result.is_ok());
        let content = std::fs::read_to_string(file_path).unwrap();
        assert!(content.contains("---\n"));

        // Cleanup
        std::fs::remove_file(file_path).unwrap();
    }

    // Test: Validate invalid characters in domain name
    #[test]
    fn test_validate_invalid_characters_in_domain() {
        let invalid_domain = "exa$mple.com";
        let result = CnameConfig::new(invalid_domain, None, None);
        assert!(matches!(
            result,
            Err(CnameError::InvalidCharacters(domain)) if domain == "exa$mple"
        ));
    }

    // Test: Validate domain with single label
    #[test]
    fn test_validate_single_label_domain() {
        let single_label = "example"; // Single label domain
        let result = CnameConfig::new(single_label, None, None);
        assert!(
        matches!(result, Err(CnameError::MalformedDomain(_))),
        "Expected MalformedDomain error for single-label domain: {}",
        single_label
    );
    }

    // Test: Generate CNAME with TTL 0 (should fail)
    #[test]
    fn test_invalid_ttl_zero() {
        let result = CnameConfig::new("example.com", Some(0), None);
        assert!(matches!(result, Err(CnameError::InvalidTtl(_))));
    }
    #[test]
    fn test_unicode_domain_handling() {
        let domain = "exámple.com"; // Contains non-ASCII characters
        let result = CnameConfig::new(domain, Some(3600), None);
        assert!(
            result.is_ok(),
            "Unicode domains should pass with IDN conversion"
        );
        if let Ok(config) = result {
            assert_eq!(config.domain, "xn--exmple-qta.com"); // Expected Punycode result
        }
    }
    #[test]
    fn test_excessive_total_domain_length() {
        let long_domain = format!(
            "{}.{}.{}",
            "a".repeat(63),
            "b".repeat(63),
            "c".repeat(130) // Total length exceeds 255 characters
        );

        let result = CnameConfig::new(&long_domain, Some(3600), None);

        assert!(
            matches!(result, Err(CnameError::ExcessiveDomainLength(_))),
            "Expected ExcessiveDomainLength error for domain: {}",
            long_domain
        );
    }

    #[test]
    fn test_custom_format_missing_variables() {
        let config = CnameConfig::new(
            "example.com",
            Some(3600),
            Some("{domain} IN CNAME".to_string()), // Missing TTL
        )
        .unwrap();

        let generator = CnameGenerator::new(config);
        let record = generator.generate();

        assert_eq!(record, "example.com IN CNAME");
    }
    #[test]
    fn test_order_preservation_in_batch_generation() {
        let configs = vec![
            CnameConfig::new("b.example.com", Some(3600), None)
                .unwrap(),
            CnameConfig::new("a.example.com", Some(3600), None)
                .unwrap(),
        ];

        let records = CnameGenerator::batch_generate(configs.clone());
        let results: Vec<_> =
            records.into_iter().filter_map(Result::ok).collect();

        // Ensure output matches input order
        assert_eq!(
            results,
            vec![
                "b.example.com 3600 IN CNAME www.b.example.com",
                "a.example.com 3600 IN CNAME www.a.example.com"
            ]
        );
    }
    #[test]
    fn test_metadata_with_invalid_ttl() {
        let mut metadata = HashMap::new();
        _ = metadata
            .insert("cname".to_string(), "example.com".to_string());
        _ = metadata.insert("ttl".to_string(), "invalid".to_string());

        let result = CnameGenerator::from_metadata(&metadata);
        assert!(matches!(result, Err(CnameError::InvalidTtl(_))));
    }

    #[test]
    fn test_custom_format_with_escapes() {
        let config = CnameConfig::new(
            "example.com",
            Some(3600),
            Some("\\{domain\\} {ttl}".to_string()),
        )
        .unwrap();
        let generator = CnameGenerator::new(config);
        let record = generator.generate();
        assert_eq!(record, "\\{domain\\} 3600");
    }

    #[test]
    fn test_error_display_variants() {
        let errors = vec![
            CnameError::EmptyDomain,
            CnameError::InvalidCharacters("test".to_string()),
            CnameError::LabelTooLong("test".to_string()),
            CnameError::MalformedDomain("test".to_string()),
            CnameError::InvalidHyphenUsage("test".to_string()),
            CnameError::InvalidTtl("test".to_string()),
            CnameError::ExcessiveDomainLength("test".to_string()),
        ];

        for err in errors {
            assert!(!err.to_string().is_empty());
        }
    }

    #[test]
    fn test_batch_generate_empty_input() {
        let records = CnameGenerator::batch_generate(vec![]);
        assert!(records.is_empty());
    }

    #[test]
    fn test_export_batch_empty_input() {
        let file_path = "test_empty.txt";

        // Attempt to export an empty batch
        let result = CnameGenerator::export_batch_to_file(
            vec![],
            file_path,
            "\n",
        );
        assert!(result.is_ok());

        // Cleanup: Remove the file after the test
        if std::fs::remove_file(file_path).is_ok() {
            println!("🗑️ File '{}' removed after the test.", file_path);
        } else {
            println!("⚠️ Could not remove file '{}'.", file_path);
        }
    }

    #[test]
    fn test_debug_implementation() {
        let config =
            CnameConfig::new("example.com", Some(3600), None).unwrap();
        let generator = CnameGenerator::new(config);
        assert!(!format!("{:?}", generator).is_empty());
    }

    #[test]
    fn test_unicode_normalization() {
        let configs = vec![
            CnameConfig::new("café.com", Some(3600), None).unwrap(),
            CnameConfig::new("cafe\u{0301}.com", Some(3600), None)
                .unwrap(),
        ];
        let records = CnameGenerator::batch_generate(configs);
        let results: Vec<_> =
            records.into_iter().filter_map(Result::ok).collect();
        assert_eq!(results[0], results[1]);
    }

    #[test]
    fn test_non_utf8_domain() {
        let domain = String::from_utf8(vec![0xFF]).unwrap_or_default();
        let result = CnameConfig::new(domain, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_metadata_with_custom_format() {
        let mut metadata = HashMap::new();
        _ = metadata
            .insert("cname".to_string(), "example.com".to_string());
        _ = metadata.insert(
            "format".to_string(),
            "{domain} CNAME {ttl}".to_string(),
        );
        let result = CnameGenerator::from_metadata(&metadata).unwrap();
        assert!(result.contains("example.com CNAME 3600"));
    }

    #[test]
    fn test_clone_and_eq() {
        let config1 =
            CnameConfig::new("example.com", Some(3600), None).unwrap();
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_error_chain() {
        let error = CnameError::InvalidTtl("test".into());
        assert!(error.source().is_none());
    }

    #[test]
    fn test_serialize_deserialize() {
        let config =
            CnameConfig::new("example.com", Some(3600), None).unwrap();
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: CnameConfig =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_generate_custom_empty_format() {
        let config = CnameConfig::new(
            "example.com",
            Some(3600),
            Some(String::new()),
        )
        .unwrap();
        let generator = CnameGenerator::new(config);
        let record = generator.generate();
        assert!(record.is_empty());
    }

    #[test]
    fn test_batch_generate_error_propagation() {
        let configs = vec![
            CnameConfig::new("example.com", Some(3600), None).unwrap(),
            CnameConfig::new("invalid..domain", Some(3600), None)
                .unwrap_or_default(),
        ];
        let results = CnameGenerator::batch_generate(configs);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_export_batch_to_file_io_error() {
        let config =
            CnameConfig::new("example.com", Some(3600), None).unwrap();
        let result = CnameGenerator::export_batch_to_file(
            vec![config],
            "/nonexistent/path/file.txt",
            "\n",
        );
        assert!(result.is_err());
    }
}
