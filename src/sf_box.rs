use std::{
    borrow::{Borrow, ToOwned},
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// An owning pointer to an SFML-allocated object.
///
/// On [`Drop`], it calls the appropriate SFML destructor for the object.
#[derive(Debug)]
pub struct SfBox<T: SfResource + ?Sized>(pub(crate) NonNull<T>);

impl<T: SfResource> SfBox<T> {
    pub(crate) fn new(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(SfBox)
    }
}

impl<T: SfResource> Deref for SfBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T: SfResource> DerefMut for SfBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T: SfResource> Borrow<T> for SfBox<T> {
    fn borrow(&self) -> &T {
        &*self
    }
}

impl<T: SfResource + ToOwned<Owned = SfBox<T>>> Clone for SfBox<T> {
    fn clone(&self) -> SfBox<T> {
        (**self).to_owned()
    }
}

impl<T: SfResource + ?Sized> Drop for SfBox<T> {
    fn drop(&mut self) {
        unsafe { self.0.as_mut().dispose() }
    }
}

pub trait Dispose {
    unsafe fn dispose(&mut self);
}

/// A resource handed out to us by SFML
///
/// Each resource type must call a different SFML destructor function before being destroyed.
/// This is implemented using traits to make implementing `SfBox` easier.
/// Behind the scenes, we have a `Dispose` trait that these types implement, but
/// that trait never needs to be implemented outside of this crate.
///
/// This trait is a public interface to the internal `Dispose` trait, so `SfBox` can be used
/// in a generic context.
pub trait SfResource: Dispose {}

impl<T: Dispose> SfResource for T {}

pub trait RawDefault {
    fn raw_default() -> *mut Self;
}

impl<T: SfResource + RawDefault> Default for SfBox<T> {
    fn default() -> Self {
        SfBox::new(T::raw_default()).expect("Failed to create default")
    }
}
