// Copyright © 2025-2026 Static Data Gen. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use oxml::{Document, EmptyElement, Newline, SerialiseOptions};
use std::io::Write;

/// Helper function to write XML element
///
/// This function takes a mutable reference to a `Write` sink, a string containing
/// the name of the element, and a string containing the value of the element.
///
/// # Arguments
///
/// * `writer` - A mutable reference to a `Write` sink.
/// * `name` - A string containing the name of the element.
/// * `value` - A string containing the value of the element.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - A result indicating success or
///   failure.
///   - `Ok(())` if the element was written successfully.
///   - `Err(Box<dyn std::error::Error>)` if an error occurred during the
///     writing process.
///
pub fn write_element<W: Write>(
    writer: &mut W,
    name: &str,
    value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !value.is_empty() {
        let mut doc = Document::empty();
        let elem = doc.append_element(doc.root(), None, name)?;
        let _ = doc.append_text(elem, value)?;
        let options = SerialiseOptions {
            indent: None,
            empty_elements: EmptyElement::Expanded,
            newline: Newline::Lf,
        };
        doc.write_xml_to(writer, &options)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::write_element;
    use std::io::Cursor;

    /// Tests that `write_element` correctly writes a non-empty XML element.
    #[test]
    fn test_write_element_with_value(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = Cursor::new(Vec::new());
        let element_name = "greeting";
        let element_value = "Hello, World!";

        write_element(&mut writer, element_name, element_value)?;

        let result = writer.into_inner();
        let result_str = String::from_utf8(result)?;

        let expected = "<greeting>Hello, World!</greeting>";
        assert_eq!(result_str, expected);

        Ok(())
    }

    /// Tests that `write_element` correctly handles an empty value.
    #[test]
    fn test_write_element_empty_value(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = Cursor::new(Vec::new());
        let element_name = "empty";

        write_element(&mut writer, element_name, "")?;

        let result = writer.into_inner();
        let result_str = String::from_utf8(result)?;

        // Expect empty since the value is empty, so no element should be written
        assert_eq!(result_str, "");

        Ok(())
    }

    /// Tests that `write_element` correctly handles special characters that need to be escaped.
    #[test]
    fn test_write_element_special_characters(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = Cursor::new(Vec::new());
        let element_name = "message";
        let element_value = "This & that < > \" '";

        write_element(&mut writer, element_name, element_value)?;

        let result = writer.into_inner();
        let result_str = String::from_utf8(result)?;

        let expected =
            "<message>This &amp; that &lt; &gt; \" '</message>";
        assert_eq!(result_str, expected);

        Ok(())
    }
}
