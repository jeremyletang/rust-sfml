use std::borrow::{Borrow, ToOwned};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

/// An owning pointer to an SFML-allocated object.
///
/// On [`Drop`], it calls the appropriate SFML destructor for the object.
#[derive(Debug)]
pub struct SfBox<T: Dispose>(pub(crate) NonNull<T>);

impl<T: Dispose> SfBox<T> {
    pub(crate) fn new(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(SfBox)
    }
}

impl<T: Dispose> Deref for SfBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T: Dispose> DerefMut for SfBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T: Dispose> Borrow<T> for SfBox<T> {
    fn borrow(&self) -> &T {
        &*self
    }
}

impl<T: Dispose + ToOwned<Owned = SfBox<T>>> Clone for SfBox<T> {
    fn clone(&self) -> SfBox<T> {
        (**self).to_owned()
    }
}

impl<T: Dispose> Drop for SfBox<T> {
    fn drop(&mut self) {
        unsafe { self.0.as_mut().dispose() }
    }
}

pub trait Dispose {
    unsafe fn dispose(&mut self);
}

pub trait RawDefault {
    fn raw_default() -> *mut Self;
}

impl<T: Dispose + RawDefault> Default for SfBox<T> {
    fn default() -> Self {
        SfBox::new(T::raw_default()).expect("Failed to create default")
    }
}
