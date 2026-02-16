// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use langweave::error::I18nError;
use std::collections::HashMap;
use std::sync::LazyLock;

static TRANSLATIONS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    let _ = m.insert("Hello", "Hello");
    let _ = m.insert("Goodbye", "Goodbye");
    let _ = m.insert("main_logger_msg", "\nPlease run `ssg --help` for more information.\n");
    let _ = m.insert("lib_banner_log_msg", "Banner printed successfully");
    let _ = m.insert("lib_args_log_msg", "Arguments processed successfully");
    let _ = m.insert("lib_server_log_msg", "Server started successfully");
    m
});

/// Translates the given text into English.
///
/// # Arguments
///
/// * `text` - The text to be translated.
///
/// # Returns
///
/// The translated string if a translation is found, or the original `text` if no
/// translation is available.
///
pub fn translate(key: &str) -> Result<String, I18nError> {
    if let Some(&translation) = TRANSLATIONS.get(key) {
        Ok(translation.to_string())
    } else {
        Err(I18nError::TranslationFailed(key.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_hello() {
        let result = translate("Hello");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello");
    }

    #[test]
    fn test_translate_goodbye() {
        let result = translate("Goodbye");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Goodbye");
    }

    #[test]
    fn test_translate_main_logger_msg() {
        let result = translate("main_logger_msg");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("--help"));
    }

    #[test]
    fn test_translate_lib_banner_log_msg() {
        let result = translate("lib_banner_log_msg");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Banner"));
    }

    #[test]
    fn test_translate_lib_args_log_msg() {
        let result = translate("lib_args_log_msg");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Arguments"));
    }

    #[test]
    fn test_translate_lib_server_log_msg() {
        let result = translate("lib_server_log_msg");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Server"));
    }

    #[test]
    fn test_translate_unknown_key() {
        let result = translate("unknown_key");
        assert!(result.is_err());
    }
}
