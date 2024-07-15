/// A trait containing all unsafe `Vec` functions used by this crate.
pub(crate) trait UnsafeVec<T> {
    /// Push to a `Vec` without checking the capacity.
    unsafe fn unsafe_push(&mut self, item: T);

    /// Extend to a `Vec` without checking the capacity.
    unsafe fn unsafe_extend(&mut self, item: &[T]);
}

impl<T> UnsafeVec<T> for Vec<T> {
    unsafe fn unsafe_push(&mut self, item: T) {
        std::ptr::write(self.as_mut_ptr().add(self.len()), item);
        self.set_len(self.len().unchecked_add(1));
    }

    unsafe fn unsafe_extend(&mut self, item: &[T]) {
        std::ptr::copy_nonoverlapping(item.as_ptr(), self.as_mut_ptr(), item.len());
        self.set_len(self.len().unchecked_add(item.len()));
    }
}
