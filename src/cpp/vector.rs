use std::marker::PhantomData;

/// Opaque handle to a C++ `std::vector<T>`.
#[repr(C)]
#[allow(missing_copy_implementations)]
pub struct CppVector<T: CppVectorItem + ?Sized> {
    _opaque: [u8; 0],
    _data: PhantomData<*mut T>,
}
impl<T: CppVectorItem> ::std::fmt::Debug for CppVector<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "CppVector::<{}>(<opaque> @ {:p})",
            std::any::type_name::<T>(),
            self
        )
    }
}

/// Define how to get the data, length, and (optionally) how to delete a C++ `std::vector<T>`
/// of this type.
///
/// # Safety
///
/// `get_data` and `get_len` must call the right C++ functions.
pub unsafe trait CppVectorItem {
    fn get_data(vec: &CppVector<Self>) -> *const Self;
    fn get_len(vec: &CppVector<Self>) -> usize;
    fn del(vec: &mut CppVector<Self>);
}

impl<T: CppVectorItem> ::std::ops::Deref for CppVector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(T::get_data(self), T::get_len(self)) }
    }
}
