use std::{
    borrow::Borrow,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// An owning pointer to a foreign-allocated object.
///
/// On [`Drop`], it calls the appropriate foreign function to dispose of the object.
///
/// Named `FBox` to avoid confusion with the Rust standard library [`Box`].
/// F stands for "foreign".
pub struct FBox<T: ?Sized>(pub(crate) NonNull<T>);

impl<T> std::fmt::Debug for FBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FBox<{}>({:p})",
            core::any::type_name::<T>(),
            self.0.as_ptr()
        )
    }
}

impl<T> FBox<T> {
    pub(crate) fn new(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(FBox)
    }
}

impl<T> Deref for FBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T> DerefMut for FBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T> Borrow<T> for FBox<T> {
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
