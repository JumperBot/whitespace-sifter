//! Sift duplicate whitespaces away in just one function call.
//! This crate **helps you** remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within a `string`.  
//! It naturally removes the whitespaces at the start and end of the `string`.
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

mod unsafe_vec;

use unsafe_vec::UnsafeVec;

/// A trait containing all `string` whitespace-sifting functions.
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
        if out.len() > 1 {
            let out_mut: &mut Vec<u8> = unsafe { out.as_mut_vec() };
            let new_out_mut_len: usize = unsafe { out_mut.len().unchecked_sub(2) };
            if *unsafe { out_mut.get_unchecked(new_out_mut_len) } == CARRIAGE_RETURN {
                unsafe { out_mut.set_len(new_out_mut_len) };
                return out;
            }
            let new_out_mut_len: usize = unsafe { out_mut.len().unchecked_sub(1) };
            if *unsafe { out_mut.get_unchecked(new_out_mut_len) } == LINE_FEED {
                unsafe { out_mut.set_len(new_out_mut_len) };
            }
        }
        out
    }
}

impl<T: AsRef<str>> WhitespaceSifter for T {}

/// A utility for `sift`.
fn sift_preallocated(bytes: &[u8], out: &mut String) {
    let mut ind: usize = 0;
    sift_trim_start(bytes, &mut ind, out);
    // Actual sifting
    let mut is_last_whitespace: bool = false;
    let mut is_last_carriage_return: bool = false;
    let mut is_last_carriage_return_line_feed: bool = false;
    while ind < bytes.len() {
        match get_char_metadata(*unsafe { bytes.get_unchecked(ind) }) {
            Character::SingleByte { data } => {
                ind = unsafe { ind.unchecked_add(1) };
                if is_ascii_whitespace(data) {
                    if data == LINE_FEED && is_last_carriage_return {
                        unsafe {
                            out.as_mut_vec().unsafe_push(LINE_FEED);
                        }
                        is_last_carriage_return = false;
                        is_last_carriage_return_line_feed = true;
                        continue;
                    }
                    if is_last_whitespace {
                        continue;
                    }
                    is_last_whitespace = true;
                } else {
                    is_last_whitespace = false;
                }
                unsafe { out.as_mut_vec().unsafe_push(data) };
                is_last_carriage_return = data == CARRIAGE_RETURN;
                is_last_carriage_return_line_feed = false;
                continue;
            }
            Character::MultiByte { len } => extend_from_bytes_with_len(bytes, &mut ind, out, len),
        }
        is_last_carriage_return = false;
        is_last_whitespace = false;
        is_last_carriage_return_line_feed = false;
    }
    // Implementation of str::trim_end()
    if is_last_carriage_return_line_feed {
        let new_out_len: usize = unsafe { out.len().unchecked_sub(2) };
        unsafe { out.as_mut_vec().set_len(new_out_len) };
        return;
    }
    sift_trim_end(out, is_last_whitespace);
}

/// A utility for `sift_preserve_newlines`.
fn sift_preallocated_until_newline(bytes: &[u8], ind: &mut usize, out: &mut String) {
    sift_trim_start(bytes, ind, out);
    // Actual sifting
    let mut is_last_whitespace: bool = false;
    let mut is_last_carriage_return: bool = false;
    while *ind < bytes.len() {
        match get_char_metadata(*unsafe { bytes.get_unchecked(*ind) }) {
            Character::SingleByte { data } => {
                *ind = unsafe { ind.unchecked_add(1) };
                if is_ascii_whitespace(data) {
                    if data == LINE_FEED {
                        // Implementation of str::trim_end()
                        let out_mut: &mut Vec<u8> = unsafe { out.as_mut_vec() };
                        if is_last_whitespace {
                            let new_out_mut_len: usize = unsafe { out_mut.len().unchecked_sub(1) };
                            unsafe { out_mut.set_len(new_out_mut_len) };
                        }
                        // Append newline
                        if is_last_carriage_return {
                            unsafe { out_mut.unsafe_push(CARRIAGE_RETURN) };
                        }
                        unsafe { out_mut.unsafe_push(LINE_FEED) };
                        return;
                    }
                    is_last_carriage_return = data == CARRIAGE_RETURN;
                    if is_last_whitespace {
                        continue;
                    }
                    is_last_whitespace = true;
                } else {
                    is_last_whitespace = false;
                }
                unsafe { out.as_mut_vec().unsafe_push(data) };
                is_last_carriage_return = data == CARRIAGE_RETURN;
                continue;
            }
            Character::MultiByte { len } => extend_from_bytes_with_len(bytes, ind, out, len),
        }
        is_last_carriage_return = false;
        is_last_whitespace = false;
    }
    sift_trim_end(out, is_last_whitespace);
}

/// A custom implementation of `str::trim_start`.
fn sift_trim_start(bytes: &[u8], ind: &mut usize, out: &mut String) {
    while *ind < bytes.len() {
        match get_char_metadata(*unsafe { bytes.get_unchecked(*ind) }) {
            Character::SingleByte { data } => {
                *ind = unsafe { ind.unchecked_add(1) };
                if !is_ascii_whitespace(data) {
                    unsafe { out.as_mut_vec().unsafe_push(data) };
                    break;
                }
            }
            Character::MultiByte { len } => {
                extend_from_bytes_with_len(bytes, ind, out, len);
                break;
            }
        }
    }
}

/// A custom implementation for `str::trim_end`.
fn sift_trim_end(out: &mut String, is_last_whitespace: bool) {
    if is_last_whitespace {
        let new_out_len: usize = unsafe { out.len().unchecked_sub(1) };
        unsafe { out.as_mut_vec().set_len(new_out_len) };
    }
}

enum Character {
    SingleByte { data: u8 },
    MultiByte { len: usize },
}

/// Binary extracted from [std](https://doc.rust-lang.org/src/core/str/validations.rs.html#36).
#[inline]
const fn get_char_metadata(first_byte: u8) -> Character {
    match first_byte {
        0b0000_0000..=0b0111_1111 => Character::SingleByte { data: first_byte },
        0b1000_0000..=0b1101_1111 => Character::MultiByte { len: 2 },
        0b1110_0000..=0b1110_1111 => Character::MultiByte { len: 3 },
        0b1111_0000..=0b1111_1111 => Character::MultiByte { len: 4 },
    }
}

#[allow(clippy::cast_possible_truncation)]
const SPACE: u8 = ' ' as u32 as u8;
#[allow(clippy::cast_possible_truncation)]
const HORIZONTAL_TAB: u8 = '\t' as u32 as u8;
#[allow(clippy::cast_possible_truncation)]
const LINE_FEED: u8 = '\n' as u32 as u8;
#[allow(clippy::cast_possible_truncation)]
const FORM_FEED: u8 = '\x0C' as u32 as u8;
#[allow(clippy::cast_possible_truncation)]
const CARRIAGE_RETURN: u8 = '\r' as u32 as u8;

/// Values extracted from [std](https://doc.rust-lang.org/src/core/char/methods.rs.html#1680).
#[inline]
const fn is_ascii_whitespace(codepoint: u8) -> bool {
    matches!(
        codepoint,
        SPACE | HORIZONTAL_TAB | LINE_FEED | FORM_FEED | CARRIAGE_RETURN
    )
}

/// A function mostly used for `Character::MultiByte` copying.
fn extend_from_bytes_with_len(bytes: &[u8], ind: &mut usize, out: &mut String, len: usize) {
    let new_ind: usize = unsafe { ind.unchecked_add(len) };
    unsafe {
        out.as_mut_vec()
            .unsafe_extend(bytes.get_unchecked(*ind..new_ind));
    }
    *ind = new_ind;
}

#[cfg(test)]
mod tests;
