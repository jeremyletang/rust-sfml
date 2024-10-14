use std::{
    borrow::Borrow,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// An owning pointer to an SFML-allocated object.
///
/// On [`Drop`], it calls the appropriate SFML destructor for the object.
pub struct SfBox<T: ?Sized>(pub(crate) NonNull<T>);

impl<T> std::fmt::Debug for SfBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SfBox<{}>({:p})",
            core::any::type_name::<T>(),
            self.0.as_ptr()
        )
    }
}

impl<T> SfBox<T> {
    pub(crate) fn new(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(SfBox)
    }
}

impl<T> Deref for SfBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T> DerefMut for SfBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T> Borrow<T> for SfBox<T> {
    fn borrow(&self) -> &T {
        self
    }
}

impl<T: ToOwned<Owned = SfBox<T>>> Clone for SfBox<T> {
    fn clone(&self) -> SfBox<T> {
        (**self).to_owned()
    }
}

impl<T: ?Sized> Drop for SfBox<T> {
    fn drop(&mut self) {
        unsafe {
            self.0.drop_in_place();
        }
    }
}

pub trait RawDefault {
    fn raw_default() -> *mut Self;
}

impl<T: RawDefault> Default for SfBox<T> {
    fn default() -> Self {
        SfBox::new(T::raw_default()).expect("Failed to create default")
    }
}
