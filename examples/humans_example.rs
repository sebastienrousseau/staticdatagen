// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # StaticDataGen Humans.txt Examples
//!
//! This program demonstrates the usage of `humans.txt` file generation
//! in the StaticDataGen library, showcasing various scenarios for
//! creating and managing website credits and documentation.

use staticdatagen::generators::humans::{
    HumansConfig, HumansGenerator,
};
use staticdatagen::models::data::HumansData;
use std::collections::HashMap;

/// ## Entry Point
///
/// Demonstrates humans.txt generation in multiple scenarios:
/// - Basic file creation
/// - Team credits
/// - Acknowledgments
/// - Social media
/// - Multilingual teams
/// - Technical stack
/// - Data validation
///
/// Returns `Result<(), Box<dyn std::error::Error>>` if any error occurs.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 StaticDataGen Humans.txt Examples\n");

    // Execute all examples
    run_example("Basic Humans.txt Example", basic_humans_example)?;
    run_example("Team Credits Example", team_credits_example)?;
    run_example("Acknowledgments Example", acknowledgments_example)?;
    run_example("Site Information Example", site_information_example)?;
    run_example("Social Media Example", social_media_example)?;
    run_example(
        "Multilingual Team Example",
        multilingual_team_example,
    )?;
    run_example("Technical Stack Example", technical_stack_example)?;
    // run_example("Validation Example", validation_example)?;

    println!("\n🎉 All examples completed successfully!");
    Ok(())
}

/// Executes an example and reports its status.
fn run_example<F>(
    title: &str,
    example: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn() -> Result<(), Box<dyn std::error::Error>>,
{
    println!("\n🦀 {}", title);
    println!("---------------------------------------------");
    if let Err(e) = example() {
        eprintln!("    ❌ Error in {}: {:?}", title, e);
    }
    Ok(())
}

/// ## Basic Example
///
/// Demonstrates creating a simple `humans.txt` file with essential data.
fn basic_humans_example() -> Result<(), Box<dyn std::error::Error>> {
    let humans_data = HumansData::new(
        "John Doe".into(),
        "Thanks to contributors!".into(),
    );

    // Convert HumansData to HumansConfig
    let config =
        HumansConfig::from_metadata(&humans_data.to_hashmap())?;
    let content = HumansGenerator::new(config).generate();
    println!("    ✅ Generated content:\n{}", content);
    Ok(())
}

/// ## Team Credits Example
///
/// Demonstrates generating team member details from metadata.
fn team_credits_example() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = HashMap::from([
        ("author".to_string(), "Jane Smith".to_string()),
        (
            "author_website".to_string(),
            "https://janesmith.dev".to_string(),
        ),
        ("author_twitter".to_string(), "@janesmith".to_string()),
        (
            "author_location".to_string(),
            "San Francisco, CA".to_string(),
        ),
    ]);

    let config = HumansConfig::from_metadata(&metadata)?;
    let content = HumansGenerator::new(config).generate();
    println!("    ✅ Generated team credits:\n{}", content);
    Ok(())
}

/// ## Acknowledgments Example
///
/// Showcases generating an acknowledgment section.
fn acknowledgments_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut humans_data = HumansData::new(
        "Project Team".into(),
        "Special thanks to contributors.".into(),
    );
    humans_data.site_last_updated = "2024-02-20".into();
    humans_data.site_components =
        "Rust, StaticDataGen, PostgreSQL".into();

    let config =
        HumansConfig::from_metadata(&humans_data.to_hashmap())?;
    let content = HumansGenerator::new(config).generate();
    println!("    ✅ Generated acknowledgments:\n{}", content);
    Ok(())
}

// Remaining examples follow the same pattern, adjusted with error handling and correct function calls.

/// ## Site Information Example
///
/// Demonstrates adding site details such as components and standards.
fn site_information_example() -> Result<(), Box<dyn std::error::Error>>
{
    let mut humans_data = HumansData::new(
        "Site Maintainer".into(),
        "Maintained by a great team.".into(),
    );
    humans_data.site_last_updated = "2024-02-20".into();
    humans_data.site_standards = "HTML5, CSS3, Web Components".into();
    humans_data.site_software = "Rust, PostgreSQL, Docker".into();

    let config =
        HumansConfig::from_metadata(&humans_data.to_hashmap())?;
    let content = HumansGenerator::new(config).generate();
    println!("    ✅ Generated site information:\n{}", content);
    Ok(())
}

/// ## Social Media Example
///
/// Generates content with social media links.
fn social_media_example() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = HashMap::from([
        ("author".to_string(), "Social Media Team".to_string()),
        ("author_twitter".to_string(), "@teamhandle".to_string()),
        (
            "author_website".to_string(),
            "https://teamsocial.com".to_string(),
        ),
    ]);

    let config = HumansConfig::from_metadata(&metadata)?;
    let content = HumansGenerator::new(config).generate();
    println!("    ✅ Generated social media content:\n{}", content);
    Ok(())
}

/// ## Multilingual Team Example
///
/// Generates multilingual team information.
fn multilingual_team_example() -> Result<(), Box<dyn std::error::Error>>
{
    let teams = [
        ("English Team", "London, UK"),
        ("Équipe française", "Paris, France"),
        ("Deutsches Team", "Berlin, Germany"),
        ("Equipo español", "Madrid, Spain"),
    ];

    for (team, location) in &teams {
        let mut humans_data = HumansData::new(
            team.to_string(),
            format!("Based in {}", location),
        );
        humans_data.author_location = location.to_string();

        let config =
            HumansConfig::from_metadata(&humans_data.to_hashmap())?;
        let content = HumansGenerator::new(config).generate();
        println!("    ✅ Team: {}\n{}", team, content);
    }
    Ok(())
}

/// ## Technical Stack Example
///
/// Demonstrates documenting the technical stack in `humans.txt`.
fn technical_stack_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut humans_data = HumansData::new(
        "Development Team".into(),
        "Thanks to the amazing dev team!".into(),
    );
    humans_data.site_components = concat!(
        "Frontend: HTML5, CSS3, JavaScript\n",
        "Backend: Rust, PostgreSQL\n",
        "Infrastructure: Docker, Kubernetes\n",
        "CI/CD: GitHub Actions"
    )
    .into();

    humans_data.site_standards =
        concat!("W3C HTML5, CSS3, ECMAScript 2021, WAI-ARIA").into();

    let config =
        HumansConfig::from_metadata(&humans_data.to_hashmap())?;
    let content = HumansGenerator::new(config).generate();
    println!("    ✅ Generated technical stack:\n{}", content);
    Ok(())
}

// ## Validation Example
//
// Validates various `HumansData` configurations.
// fn validation_example() -> Result<(), Box<dyn std::error::Error>> {
//     let cases = vec![
//         (HumansData::new("".into(), "Thanks".into()), false, "Empty author"),
//         (HumansData::new("Valid Author".into(), "Thanks".into()), true, "Valid data"),
//         {
//             let mut data = HumansData::new("Author".into(), "Thanks".into());
//             data.author_website = "invalid-url".into();
//             (data, false, "Invalid website URL")
//         },
//         {
//             let mut data = HumansData::new("Author".into(), "Thanks".into());
//             data.author_twitter = "invalid_twitter".into();
//             (data, false, "Invalid Twitter handle")
//         },
//     ];

//     for (data, should_be_valid, case) in cases {
//         let config = HumansConfig::from_metadata(data);
//         match HumansGenerator::new(config).validate() {
//             Ok(_) if should_be_valid => println!("    ✅ Valid case: {}", case),
//             Err(_) if !should_be_valid => println!("    ✅ Expected invalid case: {}", case),
//             _ => println!("    ❌ Unexpected validation result for {}", case),
//         }
//     }

//     Ok(())
// }
