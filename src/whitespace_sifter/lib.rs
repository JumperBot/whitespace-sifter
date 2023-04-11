//! Sift duplicate whitespaces away in just one function call.
//!
//! This crate helps you remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within the `&str`.  
//! Other than that, it naturally removes the whitespaces at the start and end of the `&str`.
//!
//! # Examples
//!
//! ```rust
//! use whitespace_sifter::*;
//!
//! // This prints `1.. 2.. 3.. 4.. 5..`.
//! println!("{}", sift(
//!   "1.. \n2..  \n\n3..   \n\n\n4..    \n\n\n\n5..     \n\n\n\n\n"
//! ));
//!
//! // This prints `A..\r\nB..\r\nC..\r\nD..\r\nE..`.
//! println!("{}", sift_with_carriage_return(
//!   "A..\r\n B..\r\n\r\n C..\r\n\r\n\r\n D..\r\n\r\n\r\n\r\n E..\r\n\r\n\r\n\r\n\r\n"
//! ));
//!
//! // This prints `1..\n2..\n3..\n4..\n5..`.
//! println!("{}", preserve_newline::sift(
//!   "1.. \n2..  \n\n3..   \n\n\n4..    \n\n\n\n5..     \n\n\n\n\n"
//! ));
//!
//! // This prints `A..\r\nB..\r\nC..\r\nD..\r\nE..`.
//! println!("{}", preserve_newline::sift_with_carriage_return(
//!   "A.. \r\n B.. \r\n\r\n C.. \r\n\r\n\r\n D.. \r\n\r\n\r\n\r\n E.. \r\n\r\n\r\n\r\n\r\n"
//! ));
//! ```

/// This remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within the `&str`.
///
/// If the `&str` contains carriage-returns do not use this.  
/// Use [`whitespace-sifter::sift_with_carriage_return(...)`](./fn.sift_with_carriage_return.html) instead.
pub fn sift(input: &str) -> String {
    let mut is_last_char_whitespace: bool = false;
    input
        .chars()
        .filter(|x| {
            let is_char_whitespace: bool = x.is_ascii_whitespace();
            let res: bool = !(is_char_whitespace && is_last_char_whitespace);
            is_last_char_whitespace = is_char_whitespace;
            res
        })
        .collect::<String>()
        .trim()
        .to_string()
}

/// This remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within the `&str` that contains carriage-returns.
///
/// This treats carriage-returns as just one `char` in the `&str`.  
/// If the `&str` does not contain carriage-returns do not use this.  
/// Use [`whitespace-sifter::sift(...)`](./fn.sift.html) instead.
pub fn sift_with_carriage_return(input: &str) -> String {
    let mut is_last_char_whitespace: bool = false;
    let mut is_last_char_carriage_return: bool = false;
    input
        .chars()
        .filter(|x| {
            let is_char_whitespace: bool = x.is_ascii_whitespace();
            let is_char_carriage_return: bool = x == &'\r';
            let res: bool = (is_last_char_carriage_return && x == &'\n')
                || !(is_char_whitespace && is_last_char_whitespace);
            is_last_char_whitespace = is_char_whitespace;
            is_last_char_carriage_return = is_char_carriage_return;
            res
        })
        .collect::<String>()
        .trim()
        .replace("\r\n\n", "\r\n")
}

#[cfg(feature = "preserve_newline")]
/// Sift through all the lines in the `&str` while preserving deduplicated newlines.  
/// This is only available if the `preserve_newline` feature is explicitly turned on. (default)
pub mod preserve_newline {
    /// This remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within the `&str`.
    ///
    /// If the `&str` contains carriage-returns do not use this.  
    /// Use [`whitespace-sifter::sift_with_carriage_return(...)`](./fn.sift_with_carriage_return.html) instead.
    pub fn sift(input: &str) -> String {
        input
            .split('\n')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>()
            .join("\n")
            .trim()
            .to_string()
    }

    /// This remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within the `&str` that contains carriage-returns.
    ///
    /// This treats carriage-returns as just one `char` in the `&str`.  
    /// If the `&str` does not contain carriage-returns do not use this.  
    /// Use [`whitespace-sifter::sift(...)`](./fn.sift.html) instead.
    pub fn sift_with_carriage_return(input: &str) -> String {
        input
            .split("\r\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>()
            .join("\r\n")
            .trim()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{sift, sift_with_carriage_return};

    #[test]
    fn test_sift() {
        let input: &str = &format!(
            "{}\n\n{}\n\n{}\n\n\n",
            "This\u{0020}\u{0020}is\u{0020}\u{0020}\u{0020}a\u{0020}\u{0020}sentence...",
            "With\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}some\u{0020}\u{0020}duplicate...",
            "Whitespaces."
        );
        assert_eq!(
            &sift(input),
            "This is a sentence...\nWith some duplicate...\nWhitespaces."
        );
    }

    #[test]
    fn test_sift_with_carriage_return() {
        let input: &str = &format!(
            "{}\r\n\r\n{}\r\n\r\n{}\r\n\r\n\r\n",
            "This\u{0020}\u{0020}is\u{0020}\u{0020}\u{0020}a\u{0020}\u{0020}sentence...",
            "With\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}some\u{0020}\u{0020}duplicate...",
            "Whitespaces."
        );
        assert_eq!(
            &sift_with_carriage_return(input),
            "This is a sentence...\r\nWith some duplicate...\r\nWhitespaces."
        );
    }

    #[test]
    fn test_sift_preserved() {
        let input: &str = &format!(
            "{}\n\n{}\n\n{}\n\n\n",
            "This. \n\nis. \n\na. \n\nsentence... \n\n",
            "With. \n\nsome. \n\nduplicate... \n\n",
            "Whitespaces. \n\n"
        );
        assert_eq!(
            &super::preserve_newline::sift(input),
            "This.\nis.\na.\nsentence...\nWith.\nsome.\nduplicate...\nWhitespaces."
        );
    }

    #[test]
    fn test_sift_with_carriage_return_preserved() {
        let input: &str = &format!(
            "{}\r\n\r\n{}\r\n\r\n{}\r\n\r\n\r\n",
            "This. \r\n\r\nis. \r\n\r\na. \r\n\r\nsentence... \r\n\r\n",
            "With. \r\n\r\nsome. \r\n\r\nduplicate... \r\n\r\n",
            "Whitespaces. \r\n\r\n"
        );
        assert_eq!(
            &super::preserve_newline::sift_with_carriage_return(input),
            "This.\r\nis.\r\na.\r\nsentence...\r\nWith.\r\nsome.\r\nduplicate...\r\nWhitespaces."
        );
    }
}
