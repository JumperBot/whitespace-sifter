use crate::{get_char_metadata, sift_trim_end, sift_trim_start, unsafe_custom_extend, Character};

/// A utility for `sift`.
pub(crate) fn sift_preallocated(in_ptr: *const u8, in_len: usize, out: &mut Vec<u8>) {
    let mut ind: usize = 0;
    sift_trim_start(in_ptr, in_len, &mut ind, out);
    // Actual sifting
    let mut copy_len: usize = 0;
    let mut is_last_whitespace: bool = false;
    let mut is_last_carriage_return: bool = false;
    let mut is_last_carriage_return_line_feed: bool = false;
    while ind < in_len {
        match get_char_metadata(unsafe { in_ptr.add(ind).read() }) {
            Character::NormalWhitespace => {
                ind = unsafe { ind.unchecked_add(1) };
                if is_last_whitespace {
                    unsafe {
                        unsafe_custom_extend(out, in_ptr.add(ind).sub(copy_len).sub(1), copy_len);
                    }
                    copy_len = 0;
                    continue;
                }
                is_last_whitespace = true;
                copy_len = unsafe { copy_len.unchecked_add(1) };
                is_last_carriage_return = false;
                is_last_carriage_return_line_feed = false;
                continue;
            }
            Character::CarriageReturn => {
                ind = unsafe { ind.unchecked_add(1) };
                if is_last_whitespace {
                    unsafe {
                        unsafe_custom_extend(out, in_ptr.add(ind).sub(copy_len).sub(1), copy_len);
                    }
                    copy_len = 0;
                    continue;
                }
                is_last_whitespace = true;
                copy_len = unsafe { copy_len.unchecked_add(1) };
                is_last_carriage_return = true;
                is_last_carriage_return_line_feed = false;
                continue;
            }
            Character::LineFeed => {
                ind = unsafe { ind.unchecked_add(1) };
                if is_last_carriage_return {
                    copy_len = unsafe { copy_len.unchecked_add(1) };
                    is_last_carriage_return = false;
                    is_last_carriage_return_line_feed = true;
                    continue;
                }
                if is_last_whitespace {
                    unsafe {
                        unsafe_custom_extend(out, in_ptr.add(ind).sub(copy_len).sub(1), copy_len);
                    }
                    copy_len = 0;
                    continue;
                }
                is_last_whitespace = true;
                copy_len = unsafe { copy_len.unchecked_add(1) };
                is_last_carriage_return = false;
                is_last_carriage_return_line_feed = false;
                continue;
            }
            Character::SingleByte => {
                ind = unsafe { ind.unchecked_add(1) };
                is_last_whitespace = false;
                copy_len = unsafe { copy_len.unchecked_add(1) };
                is_last_carriage_return = false;
                is_last_carriage_return_line_feed = false;
                continue;
            }
            Character::MultiByte { len } => {
                copy_len = unsafe { copy_len.unchecked_add(len as usize) };
                ind = unsafe { ind.unchecked_add(len as usize) };
            }
        }
        is_last_carriage_return = false;
        is_last_whitespace = false;
        is_last_carriage_return_line_feed = false;
    }
    unsafe {
        unsafe_custom_extend(out, in_ptr.add(ind).sub(copy_len), copy_len);
    }
    // Implementation of str::trim_end()
    if is_last_carriage_return_line_feed {
        let new_out_len: usize = unsafe { out.len().unchecked_sub(2) };
        unsafe { out.set_len(new_out_len) };
        return;
    }
    sift_trim_end(out, is_last_whitespace);
}
