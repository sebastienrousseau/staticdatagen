// Copyright © 2025 Static Data Gen.
// All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};
use std::io::Cursor;

/// Helper function to write XML element
///
/// This function takes a reference to a `Writer` object, a string containing
/// the name of the element, and a string containing the value of the element,
///
/// # Arguments
///
/// * `writer` - A reference to a `Writer` object.
/// * `name` - A string containing the name of the element.
/// * `value` - A string containing the value of the element.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - A result indicating success or
///    failure.
///    - `Ok(())` if the element was written successfully.
///    - `Err(Box<dyn std::error::Error>)` if an error occurred during the
///       writing process.
///
pub fn write_element(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    name: &str,
    value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !value.is_empty() {
        let element_start = BytesStart::new(name);
        writer.write_event(Event::Start(element_start.clone()))?;

        // Manually escape special characters
        let escaped_value = escape_xml(value);
        writer.write_event(Event::Text(BytesText::from_escaped(
            &escaped_value,
        )))?;

        let element_end = BytesEnd::new(name);
        writer.write_event(Event::End(element_end))?;
    }
    Ok(())
}

fn escape_xml(value: &str) -> String {
    value
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

#[cfg(test)]
mod tests {
    use super::write_element;
    use quick_xml::Writer;
    use std::io::Cursor;

    /// Tests that `write_element` correctly writes a non-empty XML element.
    #[test]
    fn test_write_element_with_value(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let element_name = "greeting";
        let element_value = "Hello, World!";

        write_element(&mut writer, element_name, element_value)?;

        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result)?;

        let expected = "<greeting>Hello, World!</greeting>";
        assert_eq!(result_str, expected);

        Ok(())
    }

    /// Tests that `write_element` correctly handles an empty value.
    #[test]
    fn test_write_element_empty_value(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let element_name = "empty";

        write_element(&mut writer, element_name, "")?;

        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result)?;

        // Expect empty since the value is empty, so no element should be written
        assert_eq!(result_str, "");

        Ok(())
    }

    /// Tests that `write_element` correctly handles special characters that need to be escaped.
    #[test]
    fn test_write_element_special_characters(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let element_name = "message";
        let element_value = "This & that < > \" '";

        write_element(&mut writer, element_name, element_value)?;

        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result)?;

        let expected = "<message>This &amp; that &lt; &gt; &quot; &apos;</message>";
        assert_eq!(result_str, expected);

        Ok(())
    }
}
