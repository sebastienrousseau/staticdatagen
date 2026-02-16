// Copyright © 2025 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Module for German translations.

use std::collections::HashMap;
use std::sync::LazyLock;

use langweave::error::I18nError;

static TRANSLATIONS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    let _ = m.insert("Hello", "Hallo");
    let _ = m.insert("Goodbye", "Auf Wiedersehen");
    let _ = m.insert("main_logger_msg", "\nFür weitere Informationen führen Sie bitte `ssg --help` aus.\n");
    let _ = m.insert("lib_banner_log_msg", "Banner erfolgreich gedruckt");
    let _ = m.insert("lib_args_log_msg", "Argumente erfolgreich verarbeitet");
    let _ = m.insert("lib_server_log_msg", "Server erfolgreich gestartet");
    m
});

/// Translates the given text into German.
///
/// This function looks up the translation for the given `text` in the `TRANSLATIONS` hash map.
/// If a translation is found, it returns the translated string. Otherwise, it returns
/// the original `text` as a fallback.
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
        assert_eq!(result.unwrap(), "Hallo");
    }

    #[test]
    fn test_translate_goodbye() {
        let result = translate("Goodbye");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Auf Wiedersehen");
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
        assert!(result.unwrap().contains("erfolgreich"));
    }

    #[test]
    fn test_translate_unknown_key() {
        let result = translate("unknown_key");
        assert!(result.is_err());
    }

    #[test]
    fn test_translate_lib_args_log_msg() {
        let result = translate("lib_args_log_msg");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Argumente"));
    }

    #[test]
    fn test_translate_lib_server_log_msg() {
        let result = translate("lib_server_log_msg");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Server"));
    }
}
