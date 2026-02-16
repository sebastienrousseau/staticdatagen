// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Module for French translations.

use std::collections::HashMap;
use std::sync::LazyLock;

use langweave::error::I18nError;

static TRANSLATIONS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    let _ = m.insert("Hello", "Bonjour");
    let _ = m.insert("Goodbye", "Au revoir");
    let _ = m.insert("main_logger_msg", "\nVeuillez lancer `ssg --help` pour plus d'informations.\n");
    let _ = m.insert("lib_banner_log_msg", "Bannière imprimée avec succès");
    let _ = m.insert("lib_args_log_msg", "Arguments traités avec succès");
    let _ = m.insert("lib_server_log_msg", "Serveur démarré avec succès");
    m
});

/// Translates the given text into French.
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
        assert_eq!(result.unwrap(), "Bonjour");
    }

    #[test]
    fn test_translate_goodbye() {
        let result = translate("Goodbye");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Au revoir");
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
        assert!(result.unwrap().contains("succès"));
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
        assert!(result.unwrap().contains("Arguments"));
    }

    #[test]
    fn test_translate_lib_server_log_msg() {
        let result = translate("lib_server_log_msg");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Serveur"));
    }
}
