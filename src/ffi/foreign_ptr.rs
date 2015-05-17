use std::marker::PhantomData;
use std::ops::Deref;
use std::mem;

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

/// Trait applied to containers for FFI types which can be referred to using
/// `Ref`. Implementing this trait is unsafe since it asserts that the first
/// field of the implementor is a `Foreign<Inner>`.
pub unsafe trait ForeignHolder {
	/// The inner FFI type of the ForeignHolder.
	type Inner: ForeignType;
}

/// An effective reference to some `ForeignHolder` grounded in a raw pointer
/// to the FFI type that the `ForeignHolder` holds.
pub struct Ref<'a, T: ForeignHolder + 'a> {
	ptr: *const T::Inner,
	phantom: PhantomData<&'a T>
}

impl<'a, T: ForeignHolder> Ref<'a, T> {
	/// Attempt to construct a `Ref` from the given raw pointer.
	///
    /// Returns `None` only if the provided pointer is null.
    ///
	/// Unsafe: the caller asserts that the referent of `ptr` will last
	/// for the specified lifetime.
	pub unsafe fn new(ptr: *const T::Inner) -> Option<Ref<'a, T>> {
		if ptr.is_null() {
			None
		} else {
			Some(Ref { ptr: ptr, phantom: PhantomData })
		}
	}
}

impl<'a, T: ForeignHolder> Deref for Ref<'a, T> {
	type Target = T;
	fn deref(&self) -> &T {
		unsafe { mem::transmute(&self.ptr) }
	}
}
