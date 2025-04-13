/// Push to a `Vec` without checking the capacity.
#[inline]
pub(crate) unsafe fn unsafe_push(vec: &mut Vec<u8>, item: u8) {
    unsafe {
        std::ptr::write(vec.as_mut_ptr().add(vec.len()), item);
        vec.set_len(vec.len().unchecked_add(1));
    }
}

/// Extend to a `Vec` without checking the capacity.
#[inline]
pub(crate) unsafe fn unsafe_custom_extend(vec: &mut Vec<u8>, ptr: *const u8, len: usize) {
    unsafe {
        std::ptr::copy_nonoverlapping(ptr, vec.as_mut_ptr().add(vec.len()), len);
        vec.set_len(vec.len().unchecked_add(len));
    }
}
