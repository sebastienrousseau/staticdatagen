// Copyright © 2024 StaticDataGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen Security.txt Examples
//!
//! This program demonstrates the usage of security.txt file generation
//! in the StaticDataGen library, showing various ways to create and
//! manage security policies according to RFC 9116.

use staticdatagen::models::data::SecurityData;
use staticdatagen::modules::security::{
    create_security_data, generate_security_content,
};
use std::collections::HashMap;

/// Entry point for the StaticDataGen Security.txt Examples program.
///
/// Demonstrates various security.txt file generation scenarios and shows
/// different ways to document security policies and contact information.
///
/// # Errors
///
/// Returns a `Result` containing a `Box<dyn std::error::Error>` if any error
/// occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen Security.txt Examples\n");

    basic_security_example()?;
    contact_methods_example()?;
    policy_documentation_example()?;
    expiration_handling_example()?;
    languages_example()?;
    encryption_example()?;
    acknowledgments_example()?;
    hiring_info_example()?;
    full_configuration_example()?;
    validation_example()?;

    println!("\n🎉 All security.txt examples completed successfully!");

    Ok(())
}

/// Demonstrates basic security.txt file creation.
fn basic_security_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Basic Security.txt Example");
    println!("---------------------------------------------");

    let security_data = SecurityData::new(
        vec!["https://example.com/security".to_string()],
        "2024-12-31T23:59:59Z".to_string(),
    );

    match security_data.validate() {
        Ok(_) => {
            let content = generate_security_content(&security_data);
            println!("    ✅ Generated security.txt content:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates multiple contact methods configuration.
fn contact_methods_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Contact Methods Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    let _ = metadata.insert(
        "security_contact".to_string(),
        "https://example.com/security, mailto:security@example.com, tel:+1-201-555-0123".to_string(),
    );
    let _ = metadata.insert(
        "security_expires".to_string(),
        "2024-12-31T23:59:59Z".to_string(),
    );

    let security_data = create_security_data(&metadata);
    match security_data.validate() {
        Ok(_) => {
            println!("    ✅ Contact methods configured:");
            for contact in &security_data.contact {
                println!("    📞 Contact: {}", contact);
            }
            let content = generate_security_content(&security_data);
            println!("\n    Generated content:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates security policy documentation.
fn policy_documentation_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Policy Documentation Example");
    println!("---------------------------------------------");

    let mut security_data = SecurityData::new(
        vec!["https://example.com/security".to_string()],
        "2024-12-31T23:59:59Z".to_string(),
    );

    security_data.policy =
        "https://example.com/security-policy".to_string();
    security_data.canonical =
        "https://example.com/.well-known/security.txt".to_string();

    match security_data.validate() {
        Ok(_) => {
            let content = generate_security_content(&security_data);
            println!("    ✅ Generated policy documentation:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates expiration date handling.
fn expiration_handling_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Expiration Handling Example");
    println!("---------------------------------------------");

    let dates = [
        "2024-12-31T23:59:59Z",
        "2025-06-30T23:59:59Z",
        "Tue, 20 Feb 2024 15:15:15 GMT", // RFC 2822 format
    ];

    for date in &dates {
        let security_data = SecurityData::new(
            vec!["https://example.com/security".to_string()],
            date.to_string(),
        );

        match security_data.validate() {
            Ok(_) => {
                println!("    ✅ Valid expiration date: {}", date);
                println!("    Generated content:");
                println!(
                    "{}",
                    generate_security_content(&security_data)
                );
            }
            Err(e) => println!("    ❌ Error for {}: {:?}", date, e),
        }
    }

    Ok(())
}

/// Demonstrates preferred languages configuration.
fn languages_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Languages Example");
    println!("---------------------------------------------");

    let mut security_data = SecurityData::new(
        vec!["https://example.com/security".to_string()],
        "2024-12-31T23:59:59Z".to_string(),
    );

    security_data.preferred_languages = "en, fr, de, es".to_string();

    match security_data.validate() {
        Ok(_) => {
            let content = generate_security_content(&security_data);
            println!(
                "    ✅ Generated content with language preferences:"
            );
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates encryption key configuration.
fn encryption_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Encryption Example");
    println!("---------------------------------------------");

    let mut security_data = SecurityData::new(
        vec!["security@example.com".to_string()],
        "2024-12-31T23:59:59Z".to_string(),
    );

    security_data.encryption =
        "https://example.com/pgp-key.txt".to_string();

    match security_data.validate() {
        Ok(_) => {
            let content = generate_security_content(&security_data);
            println!("    ✅ Generated content with encryption key:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates acknowledgments section configuration.
fn acknowledgments_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Acknowledgments Example");
    println!("---------------------------------------------");

    let mut security_data = SecurityData::new(
        vec!["https://example.com/security".to_string()],
        "2024-12-31T23:59:59Z".to_string(),
    );

    security_data.acknowledgments =
        "https://example.com/hall-of-fame".to_string();

    match security_data.validate() {
        Ok(_) => {
            let content = generate_security_content(&security_data);
            println!("    ✅ Generated content with acknowledgments:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates security hiring information.
fn hiring_info_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Hiring Information Example");
    println!("---------------------------------------------");

    let mut security_data = SecurityData::new(
        vec!["https://example.com/security".to_string()],
        "2024-12-31T23:59:59Z".to_string(),
    );

    security_data.hiring =
        "https://example.com/security-jobs".to_string();

    match security_data.validate() {
        Ok(_) => {
            let content = generate_security_content(&security_data);
            println!(
                "    ✅ Generated content with hiring information:"
            );
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates full security.txt configuration.
fn full_configuration_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Full Configuration Example");
    println!("---------------------------------------------");

    let security_data = SecurityData {
        contact: vec![
            "https://example.com/security".to_string(),
            "mailto:security@example.com".to_string(),
        ],
        expires: "2024-12-31T23:59:59Z".to_string(),
        acknowledgments: "https://example.com/thanks".to_string(),
        preferred_languages: "en, fr, de".to_string(),
        canonical: "https://example.com/.well-known/security.txt"
            .to_string(),
        policy: "https://example.com/security-policy".to_string(),
        hiring: "https://example.com/security-jobs".to_string(),
        encryption: "https://example.com/pgp-key.txt".to_string(),
    };

    match security_data.validate() {
        Ok(_) => {
            let content = generate_security_content(&security_data);
            println!(
                "    ✅ Generated complete security.txt configuration:"
            );
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates validation of security.txt data.
fn validation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Validation Example");
    println!("---------------------------------------------");

    let test_cases = vec![
        (
            SecurityData::new(
                vec![],
                "2024-12-31T23:59:59Z".to_string(),
            ),
            false,
            "Empty contacts",
        ),
        (
            SecurityData::new(
                vec!["https://example.com/security".to_string()],
                "2024-12-31T23:59:59Z".to_string(),
            ),
            true,
            "Valid basic data",
        ),
        (
            SecurityData::new(
                vec!["https://example.com/security".to_string()],
                "invalid-date".to_string(),
            ),
            false,
            "Invalid expiration date",
        ),
        (
            SecurityData::new(
                vec!["invalid-url".to_string()],
                "2024-12-31T23:59:59Z".to_string(),
            ),
            false,
            "Invalid contact URL",
        ),
    ];

    for (data, should_be_valid, case) in test_cases {
        match data.validate() {
            Ok(_) => {
                if should_be_valid {
                    println!("    ✅ Valid case: {}", case);
                } else {
                    println!(
                        "    ❌ Unexpected validation success: {}",
                        case
                    );
                }
            }
            Err(e) => {
                if !should_be_valid {
                    println!(
                        "    ✅ Expected validation failure: {}",
                        case
                    );
                } else {
                    println!(
                        "    ❌ Unexpected validation error for {}: {:?}",
                        case, e
                    );
                }
            }
        }
    }

    Ok(())
}
