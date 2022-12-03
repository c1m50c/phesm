//! Module containing helper functions for [`serde`].
//! 
//! # Example
//! ```rust
//! use phesm::util::serde::trim_string;
//! use serde_json::from_str;
//! 
//! #[derive(Debug, serde::Deserialize)]
//! pub struct StringData {
//!     #[serde(deserialize_with = "trim_string")]
//!     pub string: String,
//! }
//! 
//! let json = r#"{ "string": "    Hello, World!" }"#;
//! let string_data = from_str::<StringData>(json).unwrap();
//! assert_eq!(string_data.string, "Hello, World!".to_string());
//! ```

use serde::{de, Deserialize};


/// Helper function to be used with Serde's `deserialize_with` attribute to trim a [`String`] field.
/// 
/// # Example
/// ```rust
/// use phesm::util::serde::trim_string;
/// use serde_json::from_str;
/// 
/// #[derive(Debug, serde::Deserialize)]
/// pub struct StringData {
///     #[serde(deserialize_with = "trim_string")]
///     pub string: String,
/// }
/// 
/// let json = r#"{ "string": "    Hello, World!" }"#;
/// let string_data = from_str::<StringData>(json).unwrap();
/// assert_eq!(string_data.string, "Hello, World!".to_string());
/// ```
pub fn trim_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;
    return Ok(string.trim().to_string());
}


/// Helper function to be used with Serde's `deserialize_with` attribute to trim a [`String`] field wrapped in an [`Option`] enum.
/// If the [`String`] is empty after being trimmed the [`Option`] will contain a [`None`] value.
/// 
/// # Example
/// ```rust
/// use phesm::util::serde::trim_optional_string;
/// use serde_json::from_str;
/// 
/// #[derive(Debug, serde::Deserialize)]
/// pub struct StringData {
///     #[serde(deserialize_with = "trim_optional_string")]
///     pub maybe_string: Option<String>,
/// }
/// 
/// let json = r#"{ "maybe_string": "    Hello, World!" }"#;
/// let string_data = from_str::<StringData>(json).unwrap();
/// assert_eq!(string_data.maybe_string.unwrap(), "Hello, World!".to_string());
/// ```
pub fn trim_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let mut option = Option::<String>::deserialize(deserializer)?;

    if let Some(string) = option {
        let trim = string.trim().to_string();

        if trim.is_empty() {
            return Ok(None);
        }

        option = Some(trim);
    }

    return Ok(option);
}