// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Import the Uuid type from the uuid crate
use uuid::Uuid;

/// Generates a unique string.
///
/// This function generates a new unique string using UUID version 4 (random).
///
/// # Returns
///
/// A string containing the generated unique identifier.
///
/// # Examples
///
/// ```
/// use staticdatagen::utilities::uuid::generate_unique_string;
///
/// let unique_string = generate_unique_string();
/// println!("Unique string: {}", unique_string);
/// ```
pub fn generate_unique_string() -> String {
    // Generate a new UUID v4 (random) and convert it to a string
    Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_unique_string() {
        let uuid1 = generate_unique_string();
        let uuid2 = generate_unique_string();

        // UUIDs should be non-empty
        assert!(!uuid1.is_empty());
        assert!(!uuid2.is_empty());

        // UUIDs should be unique
        assert_ne!(uuid1, uuid2);

        // UUID v4 format: 8-4-4-4-12 characters
        assert_eq!(uuid1.len(), 36);
        assert_eq!(uuid1.chars().filter(|&c| c == '-').count(), 4);
    }

    #[test]
    fn test_uuid_format() {
        let uuid = generate_unique_string();
        let parts: Vec<&str> = uuid.split('-').collect();

        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);
    }
}
