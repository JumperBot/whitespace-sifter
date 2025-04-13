//! Sift duplicate whitespaces away in just one function call.
//! This crate **helps you** remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within a UTF-8 encoded `string`.  
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
mod unsafe_vec;

use character::{Character, CharacterHelper, CARRIAGE_RETURN, LINE_FEED};
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
        crate::sift_preallocated(input.as_ptr(), input.len(), unsafe { out.as_mut_vec() });
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
            crate::sift_preallocated_until_newline(in_ptr, in_len, &mut ind, out_vec);
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

/// A utility for `sift`.
fn sift_preallocated(in_ptr: *const u8, in_len: usize, out: &mut Vec<u8>) {
    let mut ind: usize = 0;
    sift_trim_start(in_ptr, in_len, &mut ind, out);
    // Actual sifting
    let mut copy_len: usize = 0;
    let mut is_last_whitespace: bool = false;
    let mut is_last_carriage_return: bool = false;
    let mut is_last_carriage_return_line_feed: bool = false;
    while ind < in_len {
        match unsafe { in_ptr.add(ind).read() }.get_char_metadata() {
            Character::SingleByte { data } => {
                ind = unsafe { ind.unchecked_add(1) };
                if data.is_ascii_whitespace() {
                    if data == LINE_FEED && is_last_carriage_return {
                        copy_len = unsafe { copy_len.unchecked_add(1) };
                        is_last_carriage_return = false;
                        is_last_carriage_return_line_feed = true;
                        continue;
                    }
                    if is_last_whitespace {
                        unsafe {
                            out.unsafe_custom_extend(
                                in_ptr.add(ind).sub(copy_len).sub(1),
                                copy_len,
                            );
                        }
                        copy_len = 0;
                        continue;
                    }
                    is_last_whitespace = true;
                } else {
                    is_last_whitespace = false;
                }
                copy_len = unsafe { copy_len.unchecked_add(1) };
                is_last_carriage_return = data == CARRIAGE_RETURN;
                is_last_carriage_return_line_feed = false;
                continue;
            }
            Character::MultiByte { len } => {
                copy_len = unsafe { copy_len.unchecked_add(len) };
                ind = unsafe { ind.unchecked_add(len) };
            }
        }
        is_last_carriage_return = false;
        is_last_whitespace = false;
        is_last_carriage_return_line_feed = false;
    }
    unsafe {
        out.unsafe_custom_extend(in_ptr.add(ind).sub(copy_len), copy_len);
    }
    // Implementation of str::trim_end()
    if is_last_carriage_return_line_feed {
        let new_out_len: usize = unsafe { out.len().unchecked_sub(2) };
        unsafe { out.set_len(new_out_len) };
        return;
    }
    sift_trim_end(out, is_last_whitespace);
}

/// A utility for `sift_preserve_newlines`.
fn sift_preallocated_until_newline(
    in_ptr: *const u8,
    in_len: usize,
    ind: &mut usize,
    out: &mut Vec<u8>,
) {
    sift_trim_start(in_ptr, in_len, ind, out);
    // Actual sifting
    let mut copy_len: usize = 0;
    let mut is_last_whitespace: bool = false;
    let mut is_last_carriage_return: bool = false;
    while *ind < in_len {
        match unsafe { in_ptr.add(*ind).read() }.get_char_metadata() {
            Character::SingleByte { data } => {
                *ind = unsafe { ind.unchecked_add(1) };
                if data.is_ascii_whitespace() {
                    if data == LINE_FEED {
                        unsafe {
                            out.unsafe_custom_extend(
                                in_ptr.add(*ind).sub(copy_len).sub(1),
                                copy_len,
                            );
                        }
                        // Implementation of str::trim_end()
                        if is_last_whitespace {
                            let new_out_mut_len: usize = unsafe { out.len().unchecked_sub(1) };
                            unsafe { out.set_len(new_out_mut_len) };
                        }
                        // Append newline
                        if is_last_carriage_return {
                            unsafe { out.unsafe_push(CARRIAGE_RETURN) };
                        }
                        unsafe { out.unsafe_push(LINE_FEED) };
                        return;
                    }
                    is_last_carriage_return = data == CARRIAGE_RETURN;
                    if is_last_whitespace {
                        unsafe {
                            out.unsafe_custom_extend(
                                in_ptr.add(*ind).sub(copy_len).sub(1),
                                copy_len,
                            );
                        }
                        copy_len = 0;
                        continue;
                    }
                    is_last_whitespace = true;
                } else {
                    is_last_whitespace = false;
                }
                copy_len = unsafe { copy_len.unchecked_add(1) };
                is_last_carriage_return = data == CARRIAGE_RETURN;
                continue;
            }
            Character::MultiByte { len } => {
                copy_len = unsafe { copy_len.unchecked_add(len) };
                *ind = unsafe { ind.unchecked_add(len) };
            }
        }
        is_last_carriage_return = false;
        is_last_whitespace = false;
    }
    unsafe {
        out.unsafe_custom_extend(in_ptr.add(*ind).sub(copy_len), copy_len);
    }
    sift_trim_end(out, is_last_whitespace);
}

/// A custom implementation of `str::trim_start`.
fn sift_trim_start(in_ptr: *const u8, in_len: usize, ind: &mut usize, out: &mut Vec<u8>) {
    while *ind < in_len {
        match unsafe { in_ptr.add(*ind).read() }.get_char_metadata() {
            Character::SingleByte { data } => {
                *ind = unsafe { ind.unchecked_add(1) };
                if !data.is_ascii_whitespace() {
                    unsafe { out.unsafe_push(data) };
                    break;
                }
            }
            Character::MultiByte { len } => {
                unsafe {
                    out.unsafe_custom_extend(in_ptr.add(*ind), len);
                }
                *ind = unsafe { ind.unchecked_add(len) };
                break;
            }
        }
    }
}

/// A custom implementation for `str::trim_end`.
fn sift_trim_end(out: &mut Vec<u8>, is_last_whitespace: bool) {
    if is_last_whitespace {
        let new_out_len: usize = unsafe { out.len().unchecked_sub(1) };
        unsafe { out.set_len(new_out_len) };
    }
}

#[cfg(test)]
mod tests;
