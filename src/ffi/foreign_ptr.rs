

/// Trait applied to FFI types which can be managed by `Foreign<T>`.
pub trait ForeignType {
    /// Drop the contents of the provided pointer. Usually implemented with a
    /// single FFI call.
    unsafe fn destroy(ptr: *mut Self);
}

/// An owned pointer to an FFI type.
///
/// `Foreign` acts as an expy for the unstable `Unique<T>` library type, with
/// additional guarantees that the contents are not null and that dropping the
/// `Foreign` will properly release its resources if dropped.
pub struct Foreign<T: ForeignType> {
    ptr: *mut T
}

impl<T: ForeignType> Foreign<T> {
    /// Attempt to construct a new `Foreign` from the given raw pointer.
    ///
    /// Returns `None` only if the provided pointer is null.
    ///
    /// This function is `unsafe`: the caller asserts that the `Foreign` can
    /// assume ownership of the contents of the provided pointer.
    pub unsafe fn new(ptr: *mut T) -> Option<Foreign<T>> {
        if ptr.is_null() {
            None
        } else {
            Some(Foreign { ptr: ptr })
        }
    }

    // Obtain a reference to the contained type.
    pub fn as_ref(&self) -> &T {
        unsafe { &*self.ptr }
    }

    // Obtain a mutable reference to the contained type.
    pub fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

impl<T: ForeignType> Drop for Foreign<T> {
    fn drop(&mut self) {
        unsafe { ForeignType::destroy(self.ptr) }
    }
}
