// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen CNAME Examples
//!
//! This program demonstrates the usage of CNAME record generation and validation
//! using the StaticDataGen generators module.

use staticdatagen::generators::cname::{CnameConfig, CnameGenerator};
use std::collections::HashMap;

/// Entry point for the StaticDataGen CNAME Examples program.
///
/// Demonstrates various CNAME record generation scenarios and validation cases.
///
/// # Errors
///
/// Returns a `Result` containing a `Box<dyn std::error::Error>` if any error
/// occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen CNAME Examples\n");

    basic_cname_example()?;
    combined_example()?;
    subdomain_example()?;
    multiple_domain_example()?;
    validation_example()?;
    metadata_generation_example()?;
    error_handling_example()?;
    batch_generation_example()?;
    file_export_example()?;
    custom_format_example()?;
    international_domain_example()?;
    international_domain_validation_example()?;
    www_redirect_example()?;
    edge_case_example()?;
    ttl_edge_cases_example()?;
    benchmark_generation()?;

    println!("\n🎉 All CNAME examples completed successfully!");

    Ok(())
}

/// Demonstrates basic CNAME record creation.
fn basic_cname_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Basic CNAME Example");
    println!("---------------------------------------------");

    let config = CnameConfig::new("example.com", None, None)?;
    let generator = CnameGenerator::new(config);
    let content = generator.generate();

    println!("    ✅ Generated CNAME record:");
    println!("    {}", content);

    Ok(())
}

/// Demonstrates combining metadata generation, custom formats, and file export.
fn combined_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Combined Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    _ = metadata.insert("cname".to_string(), "example.com".to_string());
    _ = metadata.insert("ttl".to_string(), "7200".to_string());
    _ = metadata.insert(
        "format".to_string(),
        "{domain} {ttl} CUSTOM_FORMAT {domain}".to_string(),
    );

    let file_path = "combined_cname.txt";

    let result = CnameGenerator::from_metadata(&metadata);

    match result {
        Ok(content) => {
            println!("    ✅ Combined example success:");
            println!("    📝 CNAME content:");
            println!("    {}", content);

            // Write the content to a file
            if let Err(write_error) =
                std::fs::write(file_path, &content)
            {
                println!(
                    "    ❌ Failed to write file: {}",
                    write_error
                );
            } else {
                println!("    📁 Exported to '{}'", file_path);
            }

            // Ensure the file is removed after the operation
            if let Err(remove_error) = std::fs::remove_file(file_path) {
                println!(
                    "    ❌ Failed to remove file: {}",
                    remove_error
                );
            } else {
                println!(
                    "    🗑️ File '{}' removed after the test.",
                    file_path
                );
            }
        }
        Err(e) => println!("    ❌ Combined example error: {}", e),
    }

    Ok(())
}

/// Demonstrates CNAME record creation with subdomains.
fn subdomain_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Subdomain CNAME Example");
    println!("---------------------------------------------");

    let subdomains =
        vec!["blog.example.com", "docs.example.com", "api.example.com"];

    for subdomain in subdomains {
        match CnameConfig::new(subdomain, None, None) {
            Ok(config) => {
                let generator = CnameGenerator::new(config);
                let content = generator.generate();
                println!("    ✅ Subdomain: {}", subdomain);
                println!("    📝 CNAME content:");
                println!("    {}", content);
            }
            Err(e) => println!("    ❌ Error for {}: {}", subdomain, e),
        }
    }

    Ok(())
}

/// Demonstrates handling multiple domains using metadata.
fn multiple_domain_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Multiple Domain Example");
    println!("---------------------------------------------");

    let domains = vec![
        "primary-domain.com",
        "secondary-domain.com",
        "alternate-domain.com",
    ];

    for domain in domains {
        let mut metadata = HashMap::new();
        _ = metadata.insert("cname".to_string(), domain.to_string());

        match CnameGenerator::from_metadata(&metadata) {
            Ok(content) => {
                println!("    ✅ Domain: {}", domain);
                println!("    📝 CNAME content:");
                println!("    {}", content);
            }
            Err(e) => println!("    ❌ Error for {}: {}", domain, e),
        }
    }

    Ok(())
}

/// Demonstrates CNAME validation rules.
fn validation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 CNAME Validation Example");
    println!("---------------------------------------------");

    let test_cases = vec![
        ("valid-domain.com", true),
        ("invalid_domain", false),
        (
            "toolong-label-exceeding-63-characters-should-fail.com",
            false,
        ),
        ("-invalid-start.com", false),
        ("invalid-end-.com", false),
        ("valid-numbers123.com", true),
        ("", false),
    ];

    for (domain, should_be_valid) in test_cases {
        match CnameConfig::new(domain, None, None) {
            Ok(config) => {
                if should_be_valid {
                    println!("    ✅ Valid domain: {}", domain);
                    let generator = CnameGenerator::new(config);
                    println!(
                        "    📝 Content: {}",
                        generator.generate()
                    );
                } else {
                    println!(
                        "    ❌ Unexpected validation success: {}",
                        domain
                    );
                }
            }
            Err(e) => {
                if !should_be_valid {
                    println!(
                        "    ✅ Expected validation failure: {}",
                        domain
                    );
                } else {
                    println!(
                        "    ❌ Validation error for {}: {}",
                        domain, e
                    );
                }
            }
        }
    }

    Ok(())
}

/// Demonstrates metadata-based CNAME generation.
fn metadata_generation_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Metadata Generation Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    _ = metadata.insert(
        "cname".to_string(),
        "metadata-example.com".to_string(),
    );

    match CnameGenerator::from_metadata(&metadata) {
        Ok(content) => {
            println!("    ✅ Generated from metadata:");
            println!("    📝 CNAME content:");
            println!("    {}", content);
        }
        Err(e) => println!("    ❌ Validation error: {}", e),
    }

    Ok(())
}

/// Demonstrates edge case scenarios for CNAME generation.
fn edge_case_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Edge Case Example");
    println!("---------------------------------------------");

    let edge_cases = vec![
        ("xn--d1acufc.xn--p1ai", "Punycode"),
        ("127.0.0.1", "IP Address"),
        ("localhost", "Localhost"),
    ];

    for (domain, description) in edge_cases {
        match CnameConfig::new(domain, None, None) {
            Ok(config) => {
                let generator = CnameGenerator::new(config);
                println!(
                    "    ✅ Edge case {}: {}",
                    description, domain
                );
                println!("    📝 Content: {}", generator.generate());
            }
            Err(e) => println!(
                "    ❌ Edge case {} error: {}",
                description, e
            ),
        }
    }

    Ok(())
}

/// Demonstrates error handling during CNAME record generation and validation.
fn error_handling_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Error Handling Example");
    println!("---------------------------------------------");

    // Define test cases with expected errors
    let error_cases = vec![
        ("", "Empty domain"),
        ("invalid domain.com", "Domain with space"),
        ("toolong-label-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.com", "Label exceeds 63 characters"),
        ("-startwithhyphen.com", "Starts with hyphen"),
        ("endwithhyphen-.com", "Ends with hyphen"),
        ("xn--d1acufc.xn--p1ai", "International domain (Punycode supported)"),
    ];

    for (domain, description) in error_cases {
        match CnameConfig::new(domain, None, None) {
            Ok(config) => {
                println!(
                    "    ❌ Unexpected success for {}: {}",
                    description, domain
                );
                let generator = CnameGenerator::new(config);
                println!("    📝 Content: {}", generator.generate());
            }
            Err(e) => {
                println!(
                    "    ✅ Expected error for {}: {}",
                    description, e
                );
            }
        }
    }

    // Metadata test case for missing required fields
    println!("\n📋 Testing metadata-based errors:");
    let mut incomplete_metadata = HashMap::new();
    _ = incomplete_metadata
        .insert("ttl".to_string(), "3600".to_string()); // Missing "cname"

    match CnameGenerator::from_metadata(&incomplete_metadata) {
        Ok(content) => {
            println!("    ❌ Unexpected success: {}", content)
        }
        Err(e) => println!("    ✅ Expected metadata error: {}", e),
    }

    // Invalid TTL value
    let mut invalid_ttl_metadata = HashMap::new();
    _ = invalid_ttl_metadata
        .insert("cname".to_string(), "example.com".to_string());
    _ = invalid_ttl_metadata
        .insert("ttl".to_string(), "-1".to_string());

    match CnameGenerator::from_metadata(&invalid_ttl_metadata) {
        Ok(content) => {
            println!("    ❌ Unexpected success: {}", content)
        }
        Err(e) => println!("    ✅ Expected TTL error: {}", e),
    }

    println!("🛠️ Completed error handling demonstration.");

    Ok(())
}

/// Demonstrates batch generation of CNAME records.
fn batch_generation_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Batch Generation Example");
    println!("---------------------------------------------");

    let configs = vec![
        CnameConfig::new("example.com", Some(7200), None)?,
        CnameConfig::new("blog.example.com", None, None)?,
        CnameConfig::new("docs.example.com", Some(3600), None)?,
    ];

    let records = CnameGenerator::batch_generate(configs);

    for (i, record) in records.iter().enumerate() {
        match record {
            Ok(content) => {
                println!("    ✅ Record {}: {}", i + 1, content)
            }
            Err(err) => println!(
                "    ❌ Error generating record {}: {}",
                i + 1,
                err
            ),
        }
    }

    Ok(())
}

/// Demonstrates exporting generated CNAME records to a file.
fn file_export_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 File Export Example");
    println!("---------------------------------------------");

    let file_path = "CNAME";

    let config = CnameConfig::new("example.com", Some(7200), None)?;
    let generator = CnameGenerator::new(config);

    // Export the CNAME record to a file
    match generator.export_to_file(file_path) {
        Ok(_) => {
            println!(
                "    ✅ CNAME record exported to '{}' file",
                file_path
            );

            // Remove the file after successful export
            if let Err(remove_error) = std::fs::remove_file(file_path) {
                println!(
                    "    ❌ Failed to remove file '{}': {}",
                    file_path, remove_error
                );
            } else {
                println!(
                    "    🗑️ File '{}' removed after the test.",
                    file_path
                );
            }
        }
        Err(e) => println!(
            "    ❌ Failed to export file '{}': {}",
            file_path, e
        ),
    }

    Ok(())
}

/// Demonstrates generating CNAME records with custom formats.
fn custom_format_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Custom Format Example");
    println!("---------------------------------------------");

    let custom_format = "{domain} {ttl} CUSTOM_FORMAT www.{domain}";

    let config = CnameConfig::new(
        "example.com",
        Some(7200),
        Some(custom_format.to_string()),
    )?;
    let generator = CnameGenerator::new(config);
    let content = generator.generate();

    println!("    ✅ Custom Format Applied:");
    println!("    📝 CNAME content:");
    println!("    {}", content);

    Ok(())
}

/// Demonstrates international domain name handling.
fn international_domain_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 International Domain Example");
    println!("---------------------------------------------");

    let domains = vec![
        "münchen.de",     // German
        "académie.fr",    // French
        "студент.рф",     // Russian
        "example.co.uk",  // UK
        "example.com.au", // Australia
        "example.co.jp",  // Japan
    ];

    for domain in domains {
        match CnameConfig::new(domain, None, None) {
            Ok(config) => {
                let generator = CnameGenerator::new(config);
                println!(
                    "    ✅ Valid international domain: {}",
                    domain
                );
                println!("    📝 CNAME content:");
                println!("    {}", generator.generate());
            }
            Err(e) => println!(
                "    ❌ Validation error for {}: {}",
                domain, e
            ),
        }
    }

    Ok(())
}

/// Demonstrates validation of internationalized domains (IDNs).
fn international_domain_validation_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 International Domain Validation Example");
    println!("---------------------------------------------");

    let domains = vec![
        "münchen.de",      // Valid
        "académie.fr",     // Valid
        "invalid_идn.com", // Invalid
        "студент.рф",      // Valid
    ];

    for domain in domains {
        match CnameConfig::new(domain, None, None) {
            Ok(config) => {
                println!("    ✅ Valid IDN: {}", domain);
                let generator = CnameGenerator::new(config);
                println!("    📝 Content: {}", generator.generate());
            }
            Err(e) => println!("    ❌ Error for {}: {}", domain, e),
        }
    }

    Ok(())
}

/// Demonstrates www subdomain redirect handling.
fn www_redirect_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 WWW Redirect Example");
    println!("---------------------------------------------");

    let domains = vec!["example.com", "mysite.org", "blog.net"];

    for domain in domains {
        match CnameConfig::new(domain, None, None) {
            Ok(config) => {
                let generator = CnameGenerator::new(config);
                let content = generator.generate();
                println!("    ✅ Domain: {}", domain);
                println!("    📝 CNAME records:");
                for record in content.split('\n') {
                    println!("       {}", record);
                }
            }
            Err(e) => println!("    ❌ Error for {}: {}", domain, e),
        }
    }

    Ok(())
}

/// Demonstrates edge cases for TTL values.
fn ttl_edge_cases_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 TTL Edge Cases Example");
    println!("---------------------------------------------");

    let ttl_cases = vec![
        (Some(0), "Invalid: Zero TTL (must fail)"),
        (Some(1), "Minimum Valid TTL"),
        (Some(u32::MAX), "Maximum Valid TTL"),
        (None, "Default TTL (3600 seconds)"),
        (Some(10), "Valid Small TTL"),
    ];

    for (ttl, description) in &ttl_cases {
        match CnameConfig::new("example.com", *ttl, None) {
            Ok(config) => {
                let generator = CnameGenerator::new(config);
                println!(
                    "    ✅ {}: {}",
                    description,
                    generator.generate()
                );
            }
            Err(e) => {
                if ttl == &Some(0) {
                    println!(
                        "    ✅ Expected failure for {}: {}",
                        description, e
                    );
                } else {
                    println!(
                        "    ❌ Unexpected error for {}: {}",
                        description, e
                    );
                }
            }
        }
    }

    // Explicitly test cases where TTL is negative or invalid in metadata
    println!("\n📋 Testing invalid TTL in metadata:");
    let mut invalid_ttl_metadata = HashMap::new();
    _ = invalid_ttl_metadata
        .insert("cname".to_string(), "example.com".to_string());
    _ = invalid_ttl_metadata
        .insert("ttl".to_string(), "-1".to_string()); // Invalid TTL

    match CnameGenerator::from_metadata(&invalid_ttl_metadata) {
        Ok(content) => {
            println!("    ❌ Unexpected success: {}", content)
        }
        Err(e) => println!(
            "    ✅ Expected error for invalid TTL in metadata: {}",
            e
        ),
    }

    println!("\n📋 Testing missing TTL in metadata:");
    let mut missing_ttl_metadata = HashMap::new();
    _ = missing_ttl_metadata
        .insert("cname".to_string(), "example.com".to_string()); // No TTL provided

    match CnameGenerator::from_metadata(&missing_ttl_metadata) {
        Ok(content) => {
            println!(
                "    ✅ Missing TTL defaults to 3600: {}",
                content
            );
        }
        Err(e) => {
            println!("    ❌ Unexpected error for missing TTL in metadata: {}", e);
        }
    }

    Ok(())
}

fn benchmark_generation() -> Result<(), Box<dyn std::error::Error>> {
    use rayon::prelude::*;
    use staticdatagen::generators::cname::CnameError;
    use std::time::Instant;

    println!("\n🦀 Optimized Benchmark Generation Example");
    println!("---------------------------------------------");

    let start = Instant::now();

    // Use parallel iterators for improved performance
    let configs: Vec<_> = (0..1_000_000)
        .into_par_iter() // Rayon for parallel iteration
        .map(|i| {
            // Directly create configs without validation overhead for known valid input
            Ok::<CnameConfig, CnameError>(CnameConfig {
                domain: format!("example{}.com", i),
                ttl: 3600,
                format: None,
            })
        })
        .collect::<Result<_, _>>()?;

    // Generate CNAME records in parallel
    let records: Vec<_> = configs
        .into_par_iter()
        .map(|config| CnameGenerator::new(config).generate())
        .collect();

    let duration = start.elapsed();
    println!("Generated {} records in {:?}.", records.len(), duration);

    Ok(())
}
