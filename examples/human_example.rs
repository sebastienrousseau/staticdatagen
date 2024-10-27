// Copyright © 2024 StaticDataGen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen Humans.txt Examples
//!
//! This program demonstrates the usage of humans.txt file generation
//! in the StaticDataGen library, showing various ways to create and
//! manage website credits and documentation.

use staticdatagen::models::data::HumansData;
use staticdatagen::modules::human::{
    create_human_data, generate_humans_content,
};
use std::collections::HashMap;

/// Entry point for the StaticDataGen Humans.txt Examples program.
///
/// Demonstrates various humans.txt file generation scenarios and shows
/// different ways to document website team and acknowledgments.
///
/// # Errors
///
/// Returns a `Result` containing a `Box<dyn std::error::Error>` if any error
/// occurs during the execution of the examples.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen Humans.txt Examples\n");

    basic_humans_example()?;
    team_credits_example()?;
    acknowledgments_example()?;
    site_information_example()?;
    social_media_example()?;
    multilingual_team_example()?;
    technical_stack_example()?;
    validation_example()?;

    println!("\n🎉 All humans.txt examples completed successfully!");

    Ok(())
}

/// Demonstrates basic humans.txt file creation.
fn basic_humans_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Basic Humans.txt Example");
    println!("---------------------------------------------");

    let humans_data = HumansData::new(
        "John Doe".to_string(),
        "Thanks to all contributors".to_string(),
    );

    match humans_data.validate() {
        Ok(_) => {
            let content = generate_humans_content(&humans_data);
            println!("    ✅ Generated humans.txt content:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates team credits in humans.txt.
fn team_credits_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Team Credits Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    let _ =
        metadata.insert("author".to_string(), "Jane Smith".to_string());
    let _ = metadata.insert(
        "author_website".to_string(),
        "https://janesmith.dev".to_string(),
    );
    let _ = metadata
        .insert("author_twitter".to_string(), "@janesmith".to_string());
    let _ = metadata.insert(
        "author_location".to_string(),
        "San Francisco, CA".to_string(),
    );

    let humans_data = create_human_data(&metadata);
    match humans_data.validate() {
        Ok(_) => {
            println!("    ✅ Team member added:");
            println!("    👤 Name: {}", humans_data.author);
            println!("    🌐 Website: {}", humans_data.author_website);
            println!("    🐦 Twitter: {}", humans_data.author_twitter);
            println!(
                "    📍 Location: {}",
                humans_data.author_location
            );
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates acknowledgments section.
fn acknowledgments_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Acknowledgments Example");
    println!("---------------------------------------------");

    let mut humans_data = HumansData::new(
        "Project Team".to_string(),
        concat!(
            "Special thanks to: \n",
            "- Open source contributors\n",
            "- Documentation team\n",
            "- Beta testers\n",
            "- Community members"
        )
        .to_string(),
    );

    // Add site information
    humans_data.site_components =
        "Rust, StaticDataGen, Web Standards".to_string();
    humans_data.site_last_updated = "2024-02-20".to_string();

    match humans_data.validate() {
        Ok(_) => {
            let content = generate_humans_content(&humans_data);
            println!("    ✅ Generated acknowledgments:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates site information section.
fn site_information_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Site Information Example");
    println!("---------------------------------------------");

    let mut humans_data = HumansData::new(
        "Site Maintainer".to_string(),
        "Thanks to our team".to_string(),
    );

    humans_data.site_last_updated = "2024-02-20".to_string();
    humans_data.site_standards =
        "HTML5, CSS3, Web Components".to_string();
    humans_data.site_components =
        "Rust, StaticDataGen, PostgreSQL".to_string();
    humans_data.site_software = "VS Code, Git, Docker".to_string();

    match humans_data.validate() {
        Ok(_) => {
            println!("    ✅ Site information:");
            println!(
                "    📅 Last Updated: {}",
                humans_data.site_last_updated
            );
            println!(
                "    🔧 Standards: {}",
                humans_data.site_standards
            );
            println!(
                "    🛠️ Components: {}",
                humans_data.site_components
            );
            println!("    💻 Software: {}", humans_data.site_software);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates social media information.
fn social_media_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Social Media Example");
    println!("---------------------------------------------");

    let mut metadata = HashMap::new();
    let _ = metadata
        .insert("author".to_string(), "Team Social".to_string());
    let _ = metadata.insert(
        "author_twitter".to_string(),
        "@teamhandle".to_string(),
    );
    let _ = metadata.insert(
        "author_website".to_string(),
        "https://teamsocial.com".to_string(),
    );
    let _ = metadata.insert(
        "thanks".to_string(),
        "Thanks to our social media team".to_string(),
    );

    let humans_data = create_human_data(&metadata);
    match humans_data.validate() {
        Ok(_) => {
            let content = generate_humans_content(&humans_data);
            println!("    ✅ Generated social media content:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates multilingual team information.
fn multilingual_team_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Multilingual Team Example");
    println!("---------------------------------------------");

    let teams = vec![
        ("English Team", "London, UK"),
        ("Équipe française", "Paris, France"),
        ("Deutsches Team", "Berlin, Germany"),
        ("Equipo español", "Madrid, Spain"),
    ];

    for (team, location) in teams {
        let mut humans_data = HumansData::new(
            team.to_string(),
            format!("Location: {}", location),
        );
        humans_data.author_location = location.to_string();

        match humans_data.validate() {
            Ok(_) => {
                println!("    ✅ Team: {}", team);
                println!("    📍 Location: {}", location);
            }
            Err(e) => println!("    ❌ Error for {}: {:?}", team, e),
        }
    }

    Ok(())
}

/// Demonstrates technical stack information.
fn technical_stack_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Technical Stack Example");
    println!("---------------------------------------------");

    let mut humans_data = HumansData::new(
        "Technical Team".to_string(),
        "Thanks to our development team".to_string(),
    );

    humans_data.site_components = concat!(
        "Frontend: HTML5, CSS3, JavaScript\n",
        "Backend: Rust, PostgreSQL\n",
        "Infrastructure: Docker, Kubernetes\n",
        "CI/CD: GitHub Actions"
    )
    .to_string();

    humans_data.site_standards = concat!(
        "- W3C HTML5\n",
        "- W3C CSS3\n",
        "- ECMAScript 2021\n",
        "- WAI-ARIA 1.2"
    )
    .to_string();

    match humans_data.validate() {
        Ok(_) => {
            let content = generate_humans_content(&humans_data);
            println!("    ✅ Generated technical stack information:");
            println!("{}", content);
        }
        Err(e) => println!("    ❌ Validation error: {:?}", e),
    }

    Ok(())
}

/// Demonstrates validation of humans.txt data.
fn validation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Validation Example");
    println!("---------------------------------------------");

    let test_cases = vec![
        (
            HumansData::new("".to_string(), "Thanks".to_string()),
            false,
            "Empty author",
        ),
        (
            HumansData::new(
                "Valid Author".to_string(),
                "Thanks".to_string(),
            ),
            true,
            "Valid basic data",
        ),
        (
            {
                let mut data = HumansData::new(
                    "Author".to_string(),
                    "Thanks".to_string(),
                );
                data.author_website = "invalid-url".to_string();
                data
            },
            false,
            "Invalid website URL",
        ),
        (
            {
                let mut data = HumansData::new(
                    "Author".to_string(),
                    "Thanks".to_string(),
                );
                data.author_twitter = "invalid_twitter".to_string();
                data
            },
            false,
            "Invalid Twitter handle",
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
