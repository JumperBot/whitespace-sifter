use crate::{
    get_char_metadata, sift_trim_end, sift_trim_start, unsafe_custom_extend, unsafe_push,
    Character, CARRIAGE_RETURN, LINE_FEED,
};

/// A utility for `sift_preserve_newlines`.
pub(crate) fn sift_preallocated_until_newline(
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
        match get_char_metadata(unsafe { in_ptr.add(*ind).read() }) {
            Character::LineFeed => {
                *ind = unsafe { ind.unchecked_add(1) };
                unsafe {
                    unsafe_custom_extend(out, in_ptr.add(*ind).sub(copy_len).sub(1), copy_len);
                }
                // Implementation of str::trim_end()
                if is_last_whitespace {
                    let new_out_mut_len: usize = unsafe { out.len().unchecked_sub(1) };
                    unsafe { out.set_len(new_out_mut_len) };
                }
                // Append newline
                if is_last_carriage_return {
                    unsafe { unsafe_push(out, CARRIAGE_RETURN) };
                }
                unsafe { unsafe_push(out, LINE_FEED) };
                return;
            }
            Character::CarriageReturn => {
                *ind = unsafe { ind.unchecked_add(1) };
                is_last_carriage_return = true;
                if is_last_whitespace {
                    unsafe {
                        unsafe_custom_extend(out, in_ptr.add(*ind).sub(copy_len).sub(1), copy_len);
                    }
                    copy_len = 0;
                    continue;
                }
                is_last_whitespace = true;
                copy_len = unsafe { copy_len.unchecked_add(1) };
                continue;
            }
            Character::NormalWhitespace => {
                *ind = unsafe { ind.unchecked_add(1) };
                is_last_carriage_return = false;
                if is_last_whitespace {
                    unsafe {
                        unsafe_custom_extend(out, in_ptr.add(*ind).sub(copy_len).sub(1), copy_len);
                    }
                    copy_len = 0;
                    continue;
                }
                is_last_whitespace = true;
                copy_len = unsafe { copy_len.unchecked_add(1) };
                continue;
            }
            Character::SingleByte => {
                *ind = unsafe { ind.unchecked_add(1) };
                is_last_whitespace = false;
                copy_len = unsafe { copy_len.unchecked_add(1) };
                is_last_carriage_return = false;
                continue;
            }
            Character::MultiByte { len } => {
                copy_len = unsafe { copy_len.unchecked_add(len as usize) };
                *ind = unsafe { ind.unchecked_add(len as usize) };
            }
        }
        is_last_carriage_return = false;
        is_last_whitespace = false;
    }
    unsafe {
        unsafe_custom_extend(out, in_ptr.add(*ind).sub(copy_len), copy_len);
    }
    sift_trim_end(out, is_last_whitespace);
}
