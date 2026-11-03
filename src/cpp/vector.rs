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

/// Define how to get the data, length, elements, and (optionally) how to delete a C++
/// `std::vector<T>` of this type.
///
/// # Safety
///
/// `get_data`, `get_len` and `get_at` must call the right C++ functions.
/// `get_at` must return a pointer to the element at `index`, for any `index` below `get_len`.
pub unsafe trait CppVectorItem {
    fn get_data(vec: &CppVector<Self>) -> *const Self;
    fn get_len(vec: &CppVector<Self>) -> usize;
    /// Get a pointer to the element at `index`.
    ///
    /// The default computes the address from `get_data`, which is only correct when the Rust
    /// type has the same size as the C++ element. Types that don't must override it.
    fn get_at(vec: &CppVector<Self>, index: usize) -> *const Self
    where
        Self: Sized,
    {
        unsafe { Self::get_data(vec).add(index) }
    }
    fn del(vec: &mut CppVector<Self>);
}

/// Marks element types whose Rust layout matches the C++ element, so that a slice can be
/// formed over the data of the vector.
///
/// # Safety
///
/// `Self` must have the same size, alignment and layout as the C++ element type.
pub unsafe trait CppVectorSliceable: CppVectorItem {}

impl<T: CppVectorItem> CppVector<T> {
    /// Get the number of elements in the vector
    #[must_use]
    pub fn len(&self) -> usize {
        T::get_len(self)
    }

    /// Check whether the vector has no elements
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the element at `index`, or `None` if `index` is out of bounds
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        (index < self.len()).then(|| unsafe { &*T::get_at(self, index) })
    }

    /// Iterate over the elements of the vector
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator {
        (0..self.len()).map(move |index| unsafe { &*T::get_at(self, index) })
    }
}

impl<T: CppVectorItem> ::std::ops::Index<usize> for CppVector<T> {
    type Output = T;

    /// Get the element at `index`
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    fn index(&self, index: usize) -> &T {
        self.get(index).expect("index out of bounds")
    }
}

impl<T: CppVectorSliceable> ::std::ops::Deref for CppVector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        let len = T::get_len(self);
        // An empty std::vector may have a null data pointer, which slices don't allow
        if len == 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(T::get_data(self), len) }
    }
}
