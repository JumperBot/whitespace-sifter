//! Sift duplicate whitespaces away in just one function call.
//! This crate **helps you** remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within a `string`.  
//! Other than that, it naturally removes the whitespaces at the start and end of the `string` using [`str::trim()`](https://doc.rust-lang.org/std/primitive.str.html#method.trim).
//!
//! # Examples
//!
//! ```rust
//! use whitespace_sifter::WhitespaceSifter;
//! // This prints `1.. 2.. 3.. 4.. 5..`.
//! println!(
//!     "{}",
//!     "1.. \n2..  \n\r\n\n3..   \n\n\n4..    \n\n\r\n\n\n5..     \n\n\n\n\n".sift(),
//! );
//!
//! // This prints `1..\n2..\n3..\n4..\r\n5..`.
//! println!(
//!     "{}",
//!     "1.. \n2..  \n\r\n3..   \n\n\n4..    \r\n\n\r\n\n5..     \n\n\n\n\n"
//!         .sift_preserve_newlines(),
//! );
//! ```

pub trait WhitespaceSifter: AsRef<str> {
    /// This removes duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) from a `string` implementing `AsRef<str>`.
    /// This follows the [is_ascii_whitespace](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_whitespace) implementation.
    /// This treats carriage-returns as just one `char` in the `string`.
    #[must_use]
    fn sift(&self) -> String {
        let input: &str = self.as_ref();
        let mut out: String = String::with_capacity(input.len());
        crate::sift_preallocated(input.as_bytes(), &mut out);
        out
    }

    /// This removes duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) from a `string` implementing `AsRef<str>`.
    /// This follows the [is_ascii_whitespace](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_whitespace) implementation.
    /// This preserves deduplicated newlines.
    /// This treats carriage-returns as just one `char` in the `string`.
    #[must_use]
    fn sift_preserve_newlines(&self) -> String {
        let input: &str = self.as_ref();
        let mut out: String = String::with_capacity(input.len());
        let bytes: &[u8] = input.as_bytes();
        let mut ind: usize = 0;
        while ind < bytes.len() {
            crate::sift_preallocated_until_newline(bytes, &mut ind, &mut out);
        }
        let out_mut: &mut Vec<u8> = unsafe { out.as_mut_vec() };
        if out_mut.len() > 1 {
            if u32::from(*unsafe { out_mut.get_unchecked(out_mut.len().unchecked_sub(2)) })
                == CARRIAGE_RETURN
            {
                out_mut.pop();
                out_mut.pop();
                return out;
            }
            if u32::from(*unsafe { out_mut.get_unchecked(out_mut.len().unchecked_sub(1)) })
                == LINE_FEED
            {
                out_mut.pop();
            }
        }
        out
    }
}

impl<T: AsRef<str>> WhitespaceSifter for T {}

fn sift_preallocated(bytes: &[u8], out: &mut String) {
    let mut ind: usize = 0;
    // Implementation of str::trim_start()
    while ind < bytes.len() {
        let (codepoint, len): (u32, usize) = next_codepoint(bytes, &mut ind);
        if !is_ascii_whitespace(codepoint) {
            let out_mut: &mut Vec<u8> = unsafe { out.as_mut_vec() };
            match len {
                1 => {
                    #[allow(clippy::cast_possible_truncation)]
                    {
                        out_mut.push(codepoint as u8);
                    }
                }
                _ => out_mut
                    .extend_from_slice(unsafe { bytes.get_unchecked(ind.unchecked_sub(len)..ind) }),
            }
            break;
        }
    }
    // Actual sifting
    let mut is_last_whitespace: bool = false;
    let mut is_last_carriage_return: bool = false;
    let mut is_last_carriage_return_line_feed: bool = false;
    while ind < bytes.len() {
        let (codepoint, len): (u32, usize) = next_codepoint(bytes, &mut ind);
        let is_carriage_return: bool = codepoint == CARRIAGE_RETURN;
        let is_line_feed: bool = codepoint == LINE_FEED;
        let is_whitespace: bool = is_ascii_whitespace(codepoint);
        if is_line_feed && is_last_carriage_return {
            #[allow(clippy::cast_possible_truncation)]
            unsafe { out.as_mut_vec() }.push(LINE_FEED as u8);
            is_last_carriage_return = false;
            is_last_carriage_return_line_feed = true;
            continue;
        }
        if is_whitespace && is_last_whitespace {
            continue;
        }
        let out_mut: &mut Vec<u8> = unsafe { out.as_mut_vec() };
        match len {
            1 => {
                #[allow(clippy::cast_possible_truncation)]
                {
                    out_mut.push(codepoint as u8);
                }
            }
            _ => out_mut
                .extend_from_slice(unsafe { bytes.get_unchecked(ind.unchecked_sub(len)..ind) }),
        }
        is_last_carriage_return = is_carriage_return;
        is_last_whitespace = is_whitespace;
        is_last_carriage_return_line_feed = false;
    }
    // Implementation of str::trim_end()
    if is_last_carriage_return_line_feed {
        let out_mut: &mut Vec<u8> = unsafe { out.as_mut_vec() };
        out_mut.pop();
        out_mut.pop();
        return;
    }
    if is_last_whitespace {
        let out_mut: &mut Vec<u8> = unsafe { out.as_mut_vec() };
        out_mut.pop();
    }
}

fn sift_preallocated_until_newline(bytes: &[u8], ind: &mut usize, out: &mut String) {
    // Implementation of str::trim_start()
    while *ind < bytes.len() {
        let (codepoint, len): (u32, usize) = next_codepoint(bytes, ind);
        if !is_ascii_whitespace(codepoint) {
            let out_mut: &mut Vec<u8> = unsafe { out.as_mut_vec() };
            match len {
                1 => {
                    #[allow(clippy::cast_possible_truncation)]
                    {
                        out_mut.push(codepoint as u8);
                    }
                }
                _ => out_mut.extend_from_slice(unsafe {
                    bytes.get_unchecked(ind.unchecked_sub(len)..*ind)
                }),
            }
            break;
        }
    }
    // Actual sifting
    let mut is_last_whitespace: bool = false;
    let mut is_last_carriage_return: bool = false;
    while *ind < bytes.len() {
        let (codepoint, len): (u32, usize) = next_codepoint(bytes, ind);
        let is_carriage_return: bool = codepoint == CARRIAGE_RETURN;
        let is_line_feed: bool = codepoint == LINE_FEED;
        let is_whitespace: bool = is_ascii_whitespace(codepoint);
        if is_line_feed {
            // Implementation of str::trim_end()
            let out_mut: &mut Vec<u8> = unsafe { out.as_mut_vec() };
            if is_last_whitespace {
                out_mut.pop();
            }
            // Append newline
            if is_last_carriage_return {
                #[allow(clippy::cast_possible_truncation)]
                {
                    out_mut.push(CARRIAGE_RETURN as u8);
                }
            }
            #[allow(clippy::cast_possible_truncation)]
            {
                out_mut.push(LINE_FEED as u8);
            }
            return;
        }
        is_last_carriage_return = is_carriage_return;
        if is_whitespace && is_last_whitespace {
            continue;
        }
        let out_mut: &mut Vec<u8> = unsafe { out.as_mut_vec() };
        match len {
            1 => {
                #[allow(clippy::cast_possible_truncation)]
                {
                    out_mut.push(codepoint as u8);
                }
            }
            _ => out_mut
                .extend_from_slice(unsafe { bytes.get_unchecked(ind.unchecked_sub(len)..*ind) }),
        }
        is_last_whitespace = is_whitespace;
    }
    // Implementation of str::trim_end()
    if is_last_whitespace {
        unsafe { out.as_mut_vec() }.pop();
    }
}

/// <https://doc.rust-lang.org/src/core/str/validations.rs.html#36>
fn next_codepoint(bytes: &[u8], ind: &mut usize) -> (u32, usize) {
    let first_byte: u8 = *unsafe { bytes.get_unchecked(*ind) };
    *ind = unsafe { ind.unchecked_add(1) };
    if first_byte < 128 {
        return (u32::from(first_byte), 1);
    }
    let init: u32 = utf8_first_byte(first_byte, 2);
    let second_byte: u8 = *unsafe { bytes.get_unchecked(*ind) };
    *ind = unsafe { ind.unchecked_add(1) };
    let third_byte: u8 = *unsafe { bytes.get_unchecked(*ind) };
    *ind = unsafe { ind.unchecked_add(1) };
    if first_byte >= 0xF0 {
        let fourth_byte: u8 = *unsafe { bytes.get_unchecked(*ind) };
        *ind = unsafe { ind.unchecked_add(1) };
        return (
            (init & 7) << 18
                | utf8_acc_cont_byte(
                    utf8_acc_cont_byte(u32::from(second_byte & CONT_MASK), third_byte),
                    fourth_byte,
                ),
            4,
        );
    }
    if first_byte >= 0xE0 {
        return (
            init << 12 | utf8_acc_cont_byte(u32::from(second_byte & CONT_MASK), third_byte),
            3,
        );
    }
    (utf8_acc_cont_byte(init, second_byte), 2)
}

/// <https://doc.rust-lang.org/src/core/str/validations.rs.html#11>
#[inline]
const fn utf8_first_byte(byte: u8, width: u32) -> u32 {
    (byte & (0x7F >> width)) as u32
}

/// <https://doc.rust-lang.org/src/core/str/validations.rs.html#274>
const CONT_MASK: u8 = 0b0011_1111;

/// <https://doc.rust-lang.org/src/core/str/validations.rs.html#16>
#[inline]
const fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
    (ch << 6) | (byte & CONT_MASK) as u32
}

const SPACE: u32 = ' ' as u32;
const HORIZONTAL_TAB: u32 = '\t' as u32;
const LINE_FEED: u32 = '\n' as u32;
const FORM_FEED: u32 = '\x0C' as u32;
const CARRIAGE_RETURN: u32 = '\r' as u32;

/// <https://doc.rust-lang.org/src/core/char/methods.rs.html#1680>
const fn is_ascii_whitespace(codepoint: u32) -> bool {
    matches!(
        codepoint,
        SPACE | HORIZONTAL_TAB | LINE_FEED | FORM_FEED | CARRIAGE_RETURN
    )
}

#[cfg(test)]
mod tests {
    use super::WhitespaceSifter;

    #[test]
    fn test_sift() {
        let input: String = format!(
            "{}\n\n{}\n\n{}\n\r\n\n{}\r\n\n\r\n{}\r\n\r\n{}\r\n\r\n\r\n",
            "This\u{0020}\u{0020}is\u{0020}\u{0020}\u{0020}a\u{0020}\u{0020}sentence...",
            "With\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}some\u{0020}\u{0020}duplicate...",
            "Whitespaces.",
            "This\u{0020}\u{0020}is\u{0020}\u{0020}\u{0020}a\u{0020}\u{0020}sentence...",
            "With\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}some\u{0020}\u{0020}duplicate...",
            "Whitespaces."
        );
        assert_eq!(
            &input.sift(),
            "This is a sentence...\nWith some duplicate...\nWhitespaces.\nThis is a sentence...\r\nWith some duplicate...\r\nWhitespaces."
        );
    }

    #[test]
    fn test_sift_preserved() {
        let input: String = format!(
            "{}\n\n{}\n\n{}\n\n\n{}\r\n\n\r\n{}\r\n\r\n{}\r\n\r\n\r\n",
            "This. \n\nis. \n\na. \n\nsentence... \n\n",
            "With. \n\nsome. \n\nduplicate... \n\n",
            "Whitespaces. \n\n",
            "This. \r\n\r\nis. \r\n\r\na. \r\n\r\nsentence... \r\n\r\n",
            "With. \r\n\r\nsome. \r\n\r\nduplicate... \r\n\r\n",
            "Whitespaces. \r\n\r\n"
        );
        assert_eq!(
            &input.sift_preserve_newlines(),
            "This.\nis.\na.\nsentence...\nWith.\nsome.\nduplicate...\nWhitespaces.\nThis.\r\nis.\r\na.\r\nsentence...\r\nWith.\r\nsome.\r\nduplicate...\r\nWhitespaces."
        );
        let input: String = format!(
            "{}\n\n{}\n\n{}\n\n\n{}\r\n\n\r\n{}\r\n\r\n{}\r\n\r\n\r\n",
            "This. \n\nis. \n\na. \n\nsentence... \n\n",
            "With. \n\nsome. \n\nduplicate... \n\n",
            "Whitespaces. \n\n",
            "This. \r\n\r\nis. \r\n\r\na. \r\n\r\nsentence... \r\n\r\n",
            "With. \r\n\r\nsome. \r\n\r\nduplicate... \r\n\r\n",
            "Whitespaces."
        );
        assert_eq!(
            &input.sift_preserve_newlines(),
            "This.\nis.\na.\nsentence...\nWith.\nsome.\nduplicate...\nWhitespaces.\nThis.\r\nis.\r\na.\r\nsentence...\r\nWith.\r\nsome.\r\nduplicate...\r\nWhitespaces."
        );
    }

    #[test]
    fn test_docs() {
        assert_eq!(
            &"1.. \n2..  \n\r\n\n3..   \n\n\n4..    \n\n\r\n\n\n5..     \n\n\n\n\n".sift(),
            "1.. 2.. 3.. 4.. 5.."
        );
        assert_eq!(
            &"1.. \n2..  \n\r\n3..   \n\n\n4..    \r\n\n\r\n\n5..     \n\n\n\n\n"
                .sift_preserve_newlines(),
            "1..\n2..\n3..\n4..\r\n5.."
        );
    }
}
