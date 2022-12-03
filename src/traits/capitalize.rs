//! Module containing implementations and traits for capitalizing strings.
//! 
//! # Examples
//! 
//! ```rust
//! use phesm::traits::capitalize::Capitalize;
//! 
//! let string = "capitalize".capitalize();
//! assert_eq!(string, "Capitalize".to_string());
//! ```
//! 
//! The above example using [`Capitalize`] does not work on untrimmed strings however,
//! Below is an example using the [`CapitalizeUntrimmed`] trait which capitalizes the first non-whitespace character.
//! 
//! ```rust
//! use phesm::traits::capitalize::CapitalizeUntrimmed;
//! 
//! let string = "    capitalize".capitalize_untrimmed();
//! assert_eq!(string, "    Capitalize".to_string());
//! ```


/// Trait implemented on [`String`]s & [`str`]s to capitalize the first character.
/// 
/// # Example
/// 
/// ```rust
/// use phesm::traits::capitalize::Capitalize;
/// 
/// let string = "hello, World!".capitalize();
/// assert_eq!(string, "Hello, World!".to_string());
/// 
/// // Notice that it doesn't work on a non-trimmed string
/// // To fix this issue use `CapitalizeUntrimmed` instead.
/// let untrimmed_string = " hello, World!".capitalize();
/// assert_eq!(untrimmed_string, " hello, World!".to_string());
/// ```
pub trait Capitalize {
    /// Takes a [`String`] or [`str`] and capitalizes the first character contained in side of it.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use phesm::traits::capitalize::Capitalize;
    /// 
    /// let string = "hello, World!".capitalize();
    /// assert_eq!(string, "Hello, World!".to_string());
    /// 
    /// // Notice that it doesn't work on a non-trimmed string
    /// // To fix this issue use `CapitalizeUntrimmed` instead.
    /// let untrimmed_string = " hello, World!".capitalize();
    /// assert_eq!(untrimmed_string, " hello, World!".to_string());
    /// ```
    fn capitalize(self) -> String;
}


impl Capitalize for String {
    fn capitalize(self) -> String {
        return if self.len() > 0 {
            self[..1].to_uppercase() + &self[1..]
        } else { String::new() };
    }
}


impl Capitalize for &str {
    fn capitalize(self) -> String {
        return if self.len() > 0 {
            self[..1].to_uppercase() + &self[1..]
        } else { String::new() };
    }
}


/// Trait implemented on [`String`]s & [`str`]s to capitalize the first non-whitespace character.
/// 
/// # Example
/// 
/// ```rust
/// use phesm::traits::capitalize::CapitalizeUntrimmed;
/// 
/// let string = "hello, World!".capitalize_untrimmed();
/// assert_eq!(string, "Hello, World!".to_string());
/// 
/// let untrimmed_string = " hello, World!".capitalize_untrimmed();
/// assert_eq!(untrimmed_string, " Hello, World!".to_string());
/// ```
pub trait CapitalizeUntrimmed {
    /// Takes a [`String`] or [`str`] and capitalizes the first non-whitespace character contained in side of it.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use phesm::traits::capitalize::CapitalizeUntrimmed;
    /// 
    /// let string = "hello, World!".capitalize_untrimmed();
    /// assert_eq!(string, "Hello, World!".to_string());
    /// 
    /// let untrimmed_string = " hello, World!".capitalize_untrimmed();
    /// assert_eq!(untrimmed_string, " Hello, World!".to_string());
    /// ```
    fn capitalize_untrimmed(self) -> String;
}


impl CapitalizeUntrimmed for String {
    fn capitalize_untrimmed(self) -> String {
        let idx = self.find(|c: char| !c.is_whitespace())
            .map(|i| i + 1)
            .unwrap_or(1);

        return if self.len() > 0 {
            self[..idx].to_uppercase() + &self[idx..]
        } else { String::new() };
    }
}


impl CapitalizeUntrimmed for &str {
    fn capitalize_untrimmed(self) -> String {
        let idx = self.find(|c: char| !c.is_whitespace())
            .map(|i| i + 1)
            .unwrap_or(1);

        return if self.len() > 0 {
            self[..idx].to_uppercase() + &self[idx..]
        } else { String::new() };
    }
}