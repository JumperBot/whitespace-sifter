//! Sift duplicate whitespaces away in just one function call.
//!
//! This crate helps you remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within the `&str`.  
//! Other than that, it naturally removes the whitespaces at the start and end of the `&str`.
//!
//! # Examples
//!
//! ```
//! use whitespace_sifter::*;
//!
//! // This prints `1.. 2.. 3.. 4.. 5..`.
//! println!("{}", sift("1.. \n2..  \n\n3..   \n\n\n4..    \n\n\n\n5..     \n\n\n\n\n"));
//!
//! // This prints `A..\r\nB..\r\nC..\r\nD..\r\nE..`.
//! println!("{}", sift_with_carriage_return("A..\r\n B..\r\n\r\n C..\r\n\r\n\r\n D..\r\n\r\n\r\n\r\n E..\r\n\r\n\r\n\r\n\r\n"));
//! ```

/// This remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within the `&str`.
///
/// If the `&str` contains carriage-returns do not use this.  
/// Use [`whitespace-sifter::sift_with_carriage_return(...)`](./fn.sift_with_carriage_return.html) instead.
pub fn sift(input: &str) -> String {
    let mut buf: &str = input;
    let mut out: String = String::new();
    while !buf.is_empty() {
        out.push_str(&buf[..1]);
        buf = &buf[1..];
        if buf.is_empty() {
            break;
        }
        let next: &str = &buf[..1];
        if next.trim().is_empty() {
            out.push_str(next);
        }
        buf = buf.trim();
    }
    out
}

/// This remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within the `&str` that contains carriage-returns.
///
/// This treats carriage-returns as just one `char` in the `&str`.  
/// If the `&str` does not contain carriage-returns do not use this.  
/// Use [`whitespace-sifter::sift(...)`](./fn.sift.html) instead.
pub fn sift_with_carriage_return(input: &str) -> String {
    let mut buf: &str = input;
    let mut out: String = String::new();
    while !buf.is_empty() {
        out.push_str(&buf[..1]);
        buf = &buf[1..];
        if buf.is_empty() {
            break;
        }
        let next: &str = &buf[..1];
        if next.trim().is_empty() {
            if buf.len() > 1 && next.eq("\r") && &buf[1..2] == "\n" {
                out.push_str("\r\n");
            } else {
                out.push_str(next);
            }
        }
        buf = buf.trim();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
