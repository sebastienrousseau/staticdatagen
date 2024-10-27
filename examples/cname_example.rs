// Copyright © 2024 StaticDataGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen CNAME Examples
//!
//! This program demonstrates the usage of CNAME record generation and validation
//! in the StaticDataGen library, showing various scenarios for domain configuration.

use staticdatagen::models::data::CnameData;
use staticdatagen::modules::cname::{
    create_cname_data, generate_cname_content,
};
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
    subdomain_example()?;
    multiple_domain_example()?;
    validation_example()?;
    metadata_generation_example()?;
    error_handling_example()?;
    international_domain_example()?;
    www_redirect_example()?;

    println!("\n🎉 All CNAME examples completed successfully!");

    Ok(())
}

/// Demonstrates basic CNAME record creation.
fn basic_cname_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Basic CNAME Example");
    println!("---------------------------------------------");

    let cname_data = CnameData::new("example.com".to_string());

    match cname_data.validate() {
        Ok(_) => {
            let content = generate_cname_content(&cname_data);
            println!("    ✅ Generated CNAME record:");
            println!("    {}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
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
        let cname_data = CnameData::new(subdomain.to_string());
        match cname_data.validate() {
            Ok(_) => {
                let content = generate_cname_content(&cname_data);
                println!("    ✅ Subdomain: {}", subdomain);
                println!("    📝 CNAME content:");
                println!("    {}", content);
            }
            Err(e) => {
                println!("    ❌ Error for {}: {:?}", subdomain, e)
            }
        }
    }

    Ok(())
}

/// Demonstrates handling multiple domains.
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
        let _ =
            metadata.insert("cname".to_string(), domain.to_string());

        let cname_data = create_cname_data(&metadata);
        match cname_data.validate() {
            Ok(_) => {
                println!("    ✅ Domain: {}", domain);
                println!("    📝 CNAME content:");
                println!("    {}", generate_cname_content(&cname_data));
            }
            Err(e) => println!("    ❌ Error for {}: {:?}", domain, e),
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
        ("too-long-label-exceeding-63-characters-should-fail-validation-according-to-dns-rules.com", false),
        ("-invalid-start.com", false),
        ("invalid-end-.com", false),
        ("valid-with-numbers123.com", true),
        ("", false),
    ];

    for (domain, should_be_valid) in test_cases {
        let cname_data = CnameData::new(domain.to_string());
        match cname_data.validate() {
            Ok(_) => {
                if should_be_valid {
                    println!("    ✅ Valid domain: {}", domain);
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
                    println!("    ❌ Unexpected validation error for {}: {:?}", domain, e);
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
    let _ = metadata.insert(
        "cname".to_string(),
        "metadata-example.com".to_string(),
    );
    let _ = metadata
        .insert("title".to_string(), "Example Site".to_string());
    let _ = metadata.insert(
        "description".to_string(),
        "A site using CNAME".to_string(),
    );

    let cname_data = create_cname_data(&metadata);
    match cname_data.validate() {
        Ok(_) => {
            println!("    ✅ Generated from metadata:");
            println!("    📝 CNAME content:");
            println!("    {}", generate_cname_content(&cname_data));
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates error handling scenarios.
fn error_handling_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Error Handling Example");
    println!("---------------------------------------------");

    let invalid_cases = vec![
        ("", "Empty domain"),
        ("no-tld", "Missing TLD"),
        (".invalid-start.com", "Invalid start character"),
        ("double..dot.com", "Double dots"),
        ("spaces not allowed.com", "Spaces in domain"),
        ("@invalid-chars$.com", "Invalid characters"),
    ];

    for (domain, case) in invalid_cases {
        let cname_data = CnameData::new(domain.to_string());
        match cname_data.validate() {
            Ok(_) => println!(
                "    ❌ Unexpected success for {}: {}",
                case, domain
            ),
            Err(e) => {
                println!("    ✅ Expected error for {}: {:?}", case, e)
            }
        }
    }

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
        let cname_data = CnameData::new(domain.to_string());
        match cname_data.validate() {
            Ok(_) => {
                println!(
                    "    ✅ Valid international domain: {}",
                    domain
                );
                println!("    📝 CNAME content:");
                println!("    {}", generate_cname_content(&cname_data));
            }
            Err(e) => println!(
                "    ❌ Validation error for {}: {:?}",
                domain, e
            ),
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
        let cname_data = CnameData::new(domain.to_string());
        match cname_data.validate() {
            Ok(_) => {
                let content = generate_cname_content(&cname_data);
                println!("    ✅ Domain: {}", domain);
                println!("    📝 CNAME records:");
                let records: Vec<&str> = content.split('\n').collect();
                for record in records {
                    println!("       {}", record);
                }
            }
            Err(e) => println!("    ❌ Error for {}: {:?}", domain, e),
        }
    }

    Ok(())
}
