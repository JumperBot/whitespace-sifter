//! Sift duplicate whitespaces away in just one function call.
//! This crate **helps you** remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within a UTF-8 encoded `string`.\
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

mod character;
mod sift;
mod sift_preserve_newlines;
mod unsafe_vec;

use character::{get_char_metadata, Character, CARRIAGE_RETURN, LINE_FEED};
use sift::sift_preallocated;
use sift_preserve_newlines::sift_preallocated_until_newline;
use unsafe_vec::{unsafe_custom_extend, unsafe_push};

/// A trait containing all `string` whitespace-sifting functions.
pub trait WhitespaceSifter: AsRef<str> {
    /// This removes duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) from a `string` implementing `AsRef<str>`.
    /// This follows the [is_ascii_whitespace](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_whitespace) implementation.
    /// This treats carriage-returns as just one `char` in the `string`.
    #[must_use]
    fn sift(&self) -> String {
        let input: &str = self.as_ref();
        let mut out: String = String::with_capacity(input.len());
        sift_preallocated(input.as_ptr(), input.len(), unsafe { out.as_mut_vec() });
        out
    }

    /// This removes duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) from a `string` implementing `AsRef<str>`.
    /// This follows the [is_ascii_whitespace](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_whitespace) implementation.
    /// This preserves deduplicated newlines.
    /// This treats carriage-returns as just one `char` in the `string`.
    #[must_use]
    fn sift_preserve_newlines(&self) -> String {
        let input: &str = self.as_ref();
        let in_ptr: *const u8 = input.as_ptr();
        let in_len: usize = input.len();
        let mut out: String = String::with_capacity(input.len());
        let out_vec: &mut Vec<u8> = unsafe { out.as_mut_vec() };
        let mut ind: usize = 0;
        while ind < in_len {
            sift_preallocated_until_newline(in_ptr, in_len, &mut ind, out_vec);
        }
        if out_vec.len() > 1 {
            let new_out_mut_len: usize = unsafe { out_vec.len().unchecked_sub(2) };
            if unsafe { out_vec.as_ptr().add(new_out_mut_len).read() } == CARRIAGE_RETURN {
                unsafe { out_vec.set_len(new_out_mut_len) };
                return out;
            }
            let new_out_mut_len: usize = unsafe { out_vec.len().unchecked_sub(1) };
            if unsafe { out_vec.as_ptr().add(new_out_mut_len).read() } == LINE_FEED {
                unsafe { out_vec.set_len(new_out_mut_len) };
            }
        }
        out
    }
}

impl<T: AsRef<str>> WhitespaceSifter for T {}

/// A custom implementation of `str::trim_start`.
#[allow(clippy::inline_always)]
#[inline(always)]
pub(crate) fn sift_trim_start(
    in_ptr: *const u8,
    in_len: usize,
    ind: &mut usize,
    out: &mut Vec<u8>,
) {
    while *ind < in_len {
        match get_char_metadata(unsafe { in_ptr.add(*ind).read() }) {
            Character::LineFeed | Character::CarriageReturn | Character::NormalWhitespace => {
                *ind = unsafe { ind.unchecked_add(1) };
            }
            Character::SingleByte => {
                unsafe { unsafe_push(out, in_ptr.add(*ind).read()) };
                *ind = unsafe { ind.unchecked_add(1) };
                break;
            }
            Character::MultiByte { len } => {
                unsafe {
                    unsafe_custom_extend(out, in_ptr.add(*ind), len as usize);
                }
                *ind = unsafe { ind.unchecked_add(len as usize) };
                break;
            }
        }
    }
}

/// A custom implementation for `str::trim_end`.
#[allow(clippy::inline_always)]
#[inline(always)]
pub(crate) fn sift_trim_end(out: &mut Vec<u8>, is_last_whitespace: bool) {
    if is_last_whitespace {
        let new_out_len: usize = unsafe { out.len().unchecked_sub(1) };
        unsafe { out.set_len(new_out_len) };
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod msrv_test;

#[cfg(test)]
mod compliance_test;
