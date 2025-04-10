/// A trait containing all unsafe `Vec` functions used by this crate.
pub(crate) trait UnsafeVec<T> {
    /// Push to a `Vec` without checking the capacity.
    unsafe fn unsafe_push(&mut self, item: T);

    /// Extend to a `Vec` without checking the capacity.
    unsafe fn unsafe_custom_extend(&mut self, ptr: *const T, len: usize);
}

impl<T> UnsafeVec<T> for Vec<T> {
    unsafe fn unsafe_push(&mut self, item: T) {
        std::ptr::write(self.as_mut_ptr().add(self.len()), item);
        self.set_len(self.len().unchecked_add(1));
    }

    unsafe fn unsafe_custom_extend(&mut self, ptr: *const T, len: usize) {
        std::ptr::copy_nonoverlapping(ptr, self.as_mut_ptr().add(self.len()), len);
        self.set_len(self.len().unchecked_add(len));
    }
}
