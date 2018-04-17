use std::borrow::{Borrow, ToOwned};
use std::ops::{Deref, DerefMut};

/// An owning pointer to an SFML-allocated object.
///
/// On [`Drop`], it calls the appropriate SFML destructor for the object.
#[derive(Debug)]
pub struct SfBox<T: Dispose>(pub(crate) *mut T);

impl<T: Dispose> Deref for SfBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.0 }
    }
}

impl<T: Dispose> DerefMut for SfBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
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
        unsafe { (*self.0).dispose() }
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
        SfBox(T::raw_default())
    }
}
