//! Sift duplicate whitespaces away in just one function call.
//! This crate **helps you** remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within the `&str`.  
//! Other than that, it naturally removes the whitespaces at the start and end of the `&str` using [`str::trim()`](https://doc.rust-lang.org/std/primitive.str.html#method.trim).
//!
//! # Examples
//!
//! ```rust
//! use whitespace_sifter::*;
//! // This prints `1.. 2.. 3.. 4.. 5..`.
//! println!(
//!     "{}",
//!     sift("1.. \n2..  \n\r\n\n3..   \n\n\n4..    \n\n\r\n\n\n5..     \n\n\n\n\n")
//! );
//!
//! // This prints `1..\n2..\n3..\n4..\r\n5..`.
//! println!(
//!     "{}",
//!     sift_preserve_newlines(
//!         "1.. \n2..  \n\r\n3..   \n\n\n4..    \r\n\n\r\n\n5..     \n\n\n\n\n"
//!     )
//! );
//! ```

/// This remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within the `&str`.  
/// This treats carriage-returns as just one `char` in the `&str`.  
#[must_use]
pub fn sift(input: &str) -> String {
    let mut out: String = String::with_capacity(input.len());
    sift_preallocated(input, &mut out);
    out
}

fn sift_preallocated(input: &str, out: &mut String) {
    let mut is_last_whitespace: bool = false;
    let mut is_last_carriage_return: bool = false;
    for c in input.trim().chars() {
        let is_carriage_return: bool = c == '\r';
        let is_newline: bool = c == '\n';
        let is_whitespace: bool = c.is_ascii_whitespace();
        if is_newline && is_last_carriage_return {
            out.push(c);
            is_last_carriage_return = false;
            continue;
        }
        if is_whitespace && is_last_whitespace {
            continue;
        }
        out.push(c);
        is_last_carriage_return = is_carriage_return;
        is_last_whitespace = is_whitespace;
    }
}

/// This remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within the `&str` while preserving deduplicated newlines.  
/// This treats carriage-returns as just one `char` in the `&str`.  
#[must_use]
pub fn sift_preserve_newlines(input: &str) -> String {
    let mut out: String = String::with_capacity(input.len());
    #[allow(clippy::str_split_at_newline)]
    for val in input.trim().split('\n') {
        let ends_with_carriage_return: bool = val.ends_with('\r');
        let val: &str = val.trim();
        if val.is_empty() {
            continue;
        }
        sift_preallocated(val, &mut out);
        if ends_with_carriage_return {
            out.push_str("\r\n");
            continue;
        }
        out.push('\n');
    }
    let out_len: usize = out.len();
    if out.ends_with("\r\n") {
        out.remove(out_len - 1);
        out.remove(out_len - 2);
    } else {
        out.remove(out_len - 1);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{sift, sift_preserve_newlines};

    #[test]
    fn test_sift() {
        let input: &str = &format!(
            "{}\n\n{}\n\n{}\n\r\n\n{}\r\n\n\r\n{}\r\n\r\n{}\r\n\r\n\r\n",
            "This\u{0020}\u{0020}is\u{0020}\u{0020}\u{0020}a\u{0020}\u{0020}sentence...",
            "With\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}some\u{0020}\u{0020}duplicate...",
            "Whitespaces.",
            "This\u{0020}\u{0020}is\u{0020}\u{0020}\u{0020}a\u{0020}\u{0020}sentence...",
            "With\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}some\u{0020}\u{0020}duplicate...",
            "Whitespaces."
        );
        assert_eq!(
            &sift(input),
            "This is a sentence...\nWith some duplicate...\nWhitespaces.\nThis is a sentence...\r\nWith some duplicate...\r\nWhitespaces."
        );
    }

    #[test]
    fn test_sift_preserved() {
        let input: &str = &format!(
            "{}\n\n{}\n\n{}\n\n\n{}\r\n\n\r\n{}\r\n\r\n{}\r\n\r\n\r\n",
            "This. \n\nis. \n\na. \n\nsentence... \n\n",
            "With. \n\nsome. \n\nduplicate... \n\n",
            "Whitespaces. \n\n",
            "This. \r\n\r\nis. \r\n\r\na. \r\n\r\nsentence... \r\n\r\n",
            "With. \r\n\r\nsome. \r\n\r\nduplicate... \r\n\r\n",
            "Whitespaces. \r\n\r\n"
        );
        assert_eq!(
            &sift_preserve_newlines(input),
            "This.\nis.\na.\nsentence...\nWith.\nsome.\nduplicate...\nWhitespaces.\nThis.\r\nis.\r\na.\r\nsentence...\r\nWith.\r\nsome.\r\nduplicate...\r\nWhitespaces."
        );
    }

    #[test]
    fn test_docs() {
        assert_eq!(
            sift("1.. \n2..  \n\r\n\n3..   \n\n\n4..    \n\n\r\n\n\n5..     \n\n\n\n\n"),
            "1.. 2.. 3.. 4.. 5.."
        );
        assert_eq!(
            &sift_preserve_newlines(
                "1.. \n2..  \n\r\n3..   \n\n\n4..    \r\n\n\r\n\n5..     \n\n\n\n\n"
            ),
            "1..\n2..\n3..\n4..\r\n5.."
        );
    }
}
