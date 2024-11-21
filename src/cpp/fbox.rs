use std::{
    borrow::Borrow,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// An owning pointer to a foreign-allocated object.
///
/// It doesn't manage its own allocation, rather, it just drops its content in-place,
/// counting on the [`Drop`] impl of the contained type to call the appropriate foreign
/// function to properly dispose of the value.
///
/// Named `FBox` to avoid confusion with the Rust standard library [`Box`].
/// F stands for "foreign".
pub struct FBox<T: ?Sized>(pub(crate) NonNull<T>);

impl<T: ?Sized> std::fmt::Debug for FBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FBox<{}>({:p})",
            core::any::type_name::<T>(),
            self.0.as_ptr()
        )
    }
}

// SAFETY: An `FBox` owns its contents, so it is safe to move between threads if and only if the
// contents is safe to move between threads. This matches the behaviour of `std::boxed::Box`.
unsafe impl <T: Send> Send for FBox<T> {}

// SAFETY: An `FBox` derefs to its contents, so it is safe to pass an `&FBox<T>` between threads if
// and only if it is safe to pass a reference to its contents between threads. This matches the
// behaviour of `std::boxed::Box`.
unsafe impl <T: Sync> Sync for FBox<T> {}

impl<T: ?Sized> FBox<T> {
    pub(crate) fn new(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(FBox)
    }
}

impl<T: ?Sized> Deref for FBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for FBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T: ?Sized> Borrow<T> for FBox<T> {
    fn borrow(&self) -> &T {
        self
    }
}

impl<T: ToOwned<Owned = FBox<T>>> Clone for FBox<T> {
    fn clone(&self) -> FBox<T> {
        (**self).to_owned()
    }
}

impl<T: ?Sized> Drop for FBox<T> {
    fn drop(&mut self) {
        unsafe {
            self.0.drop_in_place();
        }
    }
}

pub trait RawDefault {
    fn raw_default() -> NonNull<Self>;
}

impl<T: RawDefault> Default for FBox<T> {
    fn default() -> Self {
        FBox(T::raw_default())
    }
}
